use jreflection::*;
use std::collections::*;
use std::path::*;
use std::io::{self, BufWriter, Write};
use std::time::Instant;

const CLASSES_PLACEHOLDER : &'static str = "{CLASSES}";
const TEMPLATE_MD : &'static str = include_str!("template.md");
const TEMPLATE_HTML : &'static str = include_str!("template.html");

#[derive(Clone, Copy)]
enum Consistency<T: PartialEq> {
    None,
    Consistent(T),
    Inconsistent(T, T),
}

impl<T: PartialEq> std::default::Default for Consistency<T> {
    fn default() -> Self { Consistency::None }
}

impl<T: PartialEq> Consistency<T> {
    pub fn merge(&mut self, value: T) {
        *self = match take(self) {
            Consistency::None => Consistency::Consistent(value),
            Consistency::Consistent(v) => {
                if v == value {
                    Consistency::Consistent(v)
                } else {
                    Consistency::Inconsistent(v, value)
                }
            },
            Consistency::Inconsistent(a,b) => Consistency::Inconsistent(a,b),
        };
    }

    pub fn into_consistent(self) -> Option<T> {
        match self {
            Consistency::Consistent(v) => Some(v),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Default)]
struct Class {
    pub is_public:      Consistency<bool>,
    pub java_jdk:       u64,
    pub java_jre:       u64,
    pub aojdk_jdk:      u64,
    pub aojdk_jre:      u64,
    pub android_adk:    u64,
}

struct Classes(BTreeMap<String, Class>);

impl Classes {
    pub fn new() -> Self { Self(BTreeMap::new()) }

    pub fn injest_jdk_dir(&mut self, path: impl AsRef<Path>, set_class_bit: impl Fn(&mut Class)) {
        let path = path.as_ref();
        println!("Injesting {}", path.display());
        self.injest_src(Source::from_jdk_dir(path).unwrap(), set_class_bit);
    }

    pub fn injest_jar(&mut self, path: impl AsRef<Path>, set_class_bit: impl Fn(&mut Class)) {
        let path = path.as_ref();
        println!("Injesting {}", path.display());
        self.injest_src(Source::from_jar(path).unwrap(), set_class_bit);
    }

    fn injest_src(&mut self, src: Source, set_class_bit: impl Fn(&mut Class)) {
        src.for_each_class(|name|{
            let entry = self.0.entry(name.to_string()).or_default();
            set_class_bit(entry);
            let class = src.read_class(name).expect("Unable to read class");
            entry.is_public.merge(class.is_public());
            Ok(())
        }).unwrap();
    }

    pub fn write_markdown_to(&self, path: impl AsRef<Path>) -> io::Result<()> {
        self.write_template_to(path, TEMPLATE_MD)
    }

    pub fn write_html_to(&self, path: impl AsRef<Path>) -> io::Result<()> {
        self.write_template_to(path, TEMPLATE_HTML)
    }

    fn write_template_to(&self, path: impl AsRef<Path>, template: &str) -> io::Result<()> {
        let path = path.as_ref();
        println!("Writing {} classes to {}...", self.0.len(), path.display());
        let classes_placeholder_index = template.find(CLASSES_PLACEHOLDER).expect("{CLASSES} placeholder missing from template");
        let (template_pre, template_post) = template.split_at(classes_placeholder_index);
        let template_post = &template_post[CLASSES_PLACEHOLDER.len()..];

        let mut classes_md = BufWriter::new(std::fs::File::create(path)?);
        write!(classes_md, "{}", template_pre)?;

        let mut public = 0;
        for (name, data) in self.0.iter() {
            if !data.is_public.into_consistent().unwrap_or(true) {
                continue; // Skip non-public classes
            }

            public += 1;
            write!(classes_md, "    <tr><td>{}", name)?;
            for (col,               max) in [
                (data.java_jdk,     13),
                (data.java_jre,      8),
                (data.aojdk_jdk,    13),
                (data.aojdk_jre,    13),
                (data.android_adk,  29),
            ].iter().copied() {
                write!(classes_md, "</td><td>")?;
                Self::write_versions_column(&mut classes_md, col, max)?;
            }
            writeln!(classes_md, "</td></tr>")?;
        }

        write!(classes_md, "{}", template_post)?;

        println!("    {} were public", public);
        Ok(())
    }

    fn write_versions_column(md: &mut impl Write, versions: u64, max : u64) -> io::Result<()> {
        let mut anything = false;
        let mut bit = 0;
        while bit < 64 {
            if (1<<bit) & versions != 0 {
                // Found a set bit!
                if anything { write!(md, ", ")? }
                anything = true;

                let start = bit;
                while (bit < 64) && ((1<<bit) & versions != 0) { bit += 1; }
                let end = bit; // NOTE: non-inclusive

                if end-1 == start {
                    write!(md, "{}", start)?;
                } else if end - 1 == max {
                    write!(md, "{}+", start)?;
                } else {
                    write!(md, "{}-{}", start, end-1)?;
                }
            } else {
                bit += 1;
            }
        }
        if !anything { write!(md, " ")?; }
        Ok(())
    }
}

fn main() {
    let start = Instant::now();
    let mut classes = Classes::new();

    // Injest
    classes.injest_jdk_dir(r"C:\Program Files\AdoptOpenJDK\jre-13.0.1.9-hotspot",   |c| c.aojdk_jre |= 1 << 13);
    classes.injest_jdk_dir(r"C:\Program Files\AdoptOpenJDK\jre-12.0.2.10-hotspot",  |c| c.aojdk_jre |= 1 << 12);
    classes.injest_jdk_dir(r"C:\Program Files\AdoptOpenJDK\jre-11.0.5.10-hotspot",  |c| c.aojdk_jre |= 1 << 11);
    classes.injest_jdk_dir(r"C:\Program Files\AdoptOpenJDK\jre-8.0.232.09-hotspot", |c| c.aojdk_jre |= 1 <<  8);

    classes.injest_jdk_dir(r"C:\Program Files\AdoptOpenJDK\jdk-13.0.1.9-hotspot",   |c| c.aojdk_jdk |= 1 << 13);
    classes.injest_jdk_dir(r"C:\Program Files\AdoptOpenJDK\jdk-12.0.2.10-hotspot",  |c| c.aojdk_jdk |= 1 << 12);
    classes.injest_jdk_dir(r"C:\Program Files\AdoptOpenJDK\jdk-11.0.5.10-hotspot",  |c| c.aojdk_jdk |= 1 << 11);
    classes.injest_jdk_dir(r"C:\Program Files\AdoptOpenJDK\jdk-8.0.232.09-hotspot", |c| c.aojdk_jdk |= 1 <<  8);

    classes.injest_jdk_dir(r"C:\Program Files\Java\jdk-13.0.1",     |c| c.java_jdk |= 1 << 13);
    classes.injest_jdk_dir(r"C:\Program Files\Java\jdk1.8.0_231",   |c| c.java_jdk |= 1 <<  8);
    classes.injest_jdk_dir(r"C:\Program Files\Java\jre1.8.0_241",   |c| c.java_jre |= 1 <<  8);

    let adk = PathBuf::from(std::env::var_os("LOCALAPPDATA").expect("%LOCALAPPDATA% not set")).join(r"Android\Sdk\platforms");
    for sdk in 7..=29 {
        classes.injest_jar(adk.join(format!("android-{}\\android.jar", sdk)), |c| c.android_adk |= 1 << sdk);
    }

    // Fixup
    for c in classes.0.values_mut() {
        const JDK_8_AND_11  : u64 = (1<<8) | (1<<11);
        const JDK_8_THRU_11 : u64 = (1<<8) | (1<<9) | (1<<10) | (1<<11);

        const JDK_8_AND_13  : u64 = (1<<8) | (1<<13);
        const JDK_8_THRU_13 : u64 = (1<<8) | (1<<9) | (1<<10) | (1<<11) | (1<<12) | (1<<13);

        if c.aojdk_jre & JDK_8_AND_11 == JDK_8_AND_11 { c.aojdk_jre |= JDK_8_THRU_11; }
        if c.aojdk_jdk & JDK_8_AND_11 == JDK_8_AND_11 { c.aojdk_jdk |= JDK_8_THRU_11; }
        if c.java_jdk  & JDK_8_AND_13 == JDK_8_AND_13 { c.java_jdk  |= JDK_8_THRU_13; }
    }

    // Output
    classes.write_markdown_to("classes.md").expect("Failed to write classes.md");
    classes.write_html_to("classes.html").expect("Failed to write classes.html");
    let end = Instant::now();
    println!("Took {:?}", end-start);
}

// Polyfill std::mem::take (added in 1.40.0, current MSRV 1.36.0)
fn take<T>(dest: &mut T) -> T where T: Default {
    std::mem::replace(dest, T::default())
}
