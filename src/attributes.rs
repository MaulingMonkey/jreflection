use crate::*;
use crate::io::be::*;

use std::io::{self, Read};



/// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7
#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub(crate) enum Attribute {
    /// [Java SE 7 &sect; 4.7.2](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7.2)
    ConstantValue(field::Constant),

    /// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7.3
    Code { #[doc(hidden)] __nyi: () },

    /// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7.4
    StackMapTable { #[doc(hidden)] __nyi: () },

    /// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7.5
    Exceptions { #[doc(hidden)] __nyi: () },

    /// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7.6
    InnerClasses { #[doc(hidden)] __nyi: () },

    /// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7.7
    EnclosingMethod { #[doc(hidden)] __nyi: () },

    /// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7.8
    Synthetic { #[doc(hidden)] __nyi: () },

    /// [Java SE 7 &sect; 4.3.4](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.3.4): Signatures
    /// 
    /// [Java SE 7 &sect; 4.7.9](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7.9): The Signature Attribute
    /// 
    /// # Examples
    /// 
    /// * `"(Lorg/graalvm/compiler/virtual/phases/ea/PartialEscapeBlockState<TT;>;)V"`
    /// * `"<E:Ljava/lang/Object;>Ljava/util/Collections$UnmodifiableSet<TE;>;Ljava/util/SortedSet<TE;>;Ljava/io/Serializable;"`
    /// * `"Ljava/lang/Object;Ljava/security/PrivilegedExceptionAction<Ljava/lang/Boolean;>;"`
    /// 
    /// Note that a vanilla java class type starts with `L` and generic types start with `T`.
    Signature(String),

    /// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7.10
    SourceFile { #[doc(hidden)] __nyi: () },

    /// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7.11
    SourceDebugExtension { #[doc(hidden)] __nyi: () },

    /// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7.12
    LineNumberTable { #[doc(hidden)] __nyi: () },

    /// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7.13
    LocalVariableTable { #[doc(hidden)] __nyi: () },

    /// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7.14
    LocalVariableTypeTable { #[doc(hidden)] __nyi: () },

    /// [Java SE 7 &sect; 4.7.15](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7.15)
    Deprecated { #[doc(hidden)] __in_case_of_extension_break_glass: () },

    /// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7.16
    RuntimeVisibleAnnotations { #[doc(hidden)] __nyi: () },

    /// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7.17
    RuntimeInvisibleAnnotations { #[doc(hidden)] __nyi: () },

    /// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7.18
    RuntimeVisibleParameterAnnotations { #[doc(hidden)] __nyi: () },

    /// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7.19
    RuntimeInvisibleParameterAnnotations { #[doc(hidden)] __nyi: () },

    /// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7.20
    AnnotationDefault { #[doc(hidden)] __nyi: () },

    /// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7.21
    BootstrapMethods { #[doc(hidden)] __nyi: () },

    /// An unrecognized attribute was used!
    Unknown,

    #[doc(hidden)] __NonExhaustive,
}

impl Attribute {
    pub(crate) fn read(read: &mut impl Read, constants: &Constants) -> io::Result<Self> {
        let attribute_name_index    = read_u2(read)?;
        let attribute_length        = read_u4(read)? as usize;

        let name = constants.get_utf8(attribute_name_index)?;
        match name {
            "ConstantValue" => {
                // https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7.2
                io_assert!(attribute_length == 2);
                let constantvalue_index = read_u2(read)?;
                let constant = constants.get(constantvalue_index)?;
                match constant {
                    Constant::Long(value)               => Ok(Attribute::ConstantValue(field::Constant::Long(*value))),
                    Constant::Float(value)              => Ok(Attribute::ConstantValue(field::Constant::Float(*value))),
                    Constant::Double(value)             => Ok(Attribute::ConstantValue(field::Constant::Double(*value))),
                    Constant::Integer(value)            => Ok(Attribute::ConstantValue(field::Constant::Integer(*value))),
                    Constant::String { string_index }   => Ok(Attribute::ConstantValue(field::Constant::String(constants.get_utf8_possibly_invalid(*string_index)?.map(|s| s.to_owned())))),
                    c                                   => io_data_err!("Expected Constant::{{Long, Float, Double, Integer, String}}, got {:?}", c),
                }
            },
            "Signature" => {
                // https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7.9
                io_assert!(attribute_length == 2);
                let signature_index = read_u2(read)?;
                let constant = constants.get_utf8(signature_index)?;
                Ok(Attribute::Signature(constant.to_string()))
            },
            "Deprecated" => {
                // https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7.15
                // 
                // attribute_length should be 0 according to the docs... but if it's ever extended with more info, it'd
                // presumably be a semi-ignorable message that shouldn't result in an error here, so I just silently
                // skip error handling here.
                read_ignore(read, attribute_length)?;
                Ok(Attribute::Deprecated {__in_case_of_extension_break_glass:()})
            },

            // Unimplemented attributes
            "Code"                                  => { read_ignore(read, attribute_length)?; Ok(Attribute::Code                                  {__nyi:()}) },
            "StackMapTable"                         => { read_ignore(read, attribute_length)?; Ok(Attribute::StackMapTable                         {__nyi:()}) },
            "Exceptions"                            => { read_ignore(read, attribute_length)?; Ok(Attribute::Exceptions                            {__nyi:()}) },
            "InnerClasses"                          => { read_ignore(read, attribute_length)?; Ok(Attribute::InnerClasses                          {__nyi:()}) },
            "EnclosingMethod"                       => { read_ignore(read, attribute_length)?; Ok(Attribute::EnclosingMethod                       {__nyi:()}) },
            "Synthetic"                             => { read_ignore(read, attribute_length)?; Ok(Attribute::Synthetic                             {__nyi:()}) },
            "SourceFile"                            => { read_ignore(read, attribute_length)?; Ok(Attribute::SourceFile                            {__nyi:()}) },
            "SourceDebugExtension"                  => { read_ignore(read, attribute_length)?; Ok(Attribute::SourceDebugExtension                  {__nyi:()}) },
            "LineNumberTable"                       => { read_ignore(read, attribute_length)?; Ok(Attribute::LineNumberTable                       {__nyi:()}) },
            "LocalVariableTable"                    => { read_ignore(read, attribute_length)?; Ok(Attribute::LocalVariableTable                    {__nyi:()}) },
            "LocalVariableTypeTable"                => { read_ignore(read, attribute_length)?; Ok(Attribute::LocalVariableTypeTable                {__nyi:()}) },
            "RuntimeVisibleAnnotations"             => { read_ignore(read, attribute_length)?; Ok(Attribute::RuntimeVisibleAnnotations             {__nyi:()}) },
            "RuntimeInvisibleAnnotations"           => { read_ignore(read, attribute_length)?; Ok(Attribute::RuntimeInvisibleAnnotations           {__nyi:()}) },
            "RuntimeVisibleParameterAnnotations"    => { read_ignore(read, attribute_length)?; Ok(Attribute::RuntimeVisibleParameterAnnotations    {__nyi:()}) },
            "RuntimeInvisibleParameterAnnotations"  => { read_ignore(read, attribute_length)?; Ok(Attribute::RuntimeInvisibleParameterAnnotations  {__nyi:()}) },
            "AnnotationDefault"                     => { read_ignore(read, attribute_length)?; Ok(Attribute::AnnotationDefault                     {__nyi:()}) },
            "BootstrapMethods"                      => { read_ignore(read, attribute_length)?; Ok(Attribute::BootstrapMethods                      {__nyi:()}) },
            _                                       => { read_ignore(read, attribute_length)?; Ok(Attribute::Unknown) },
        }
    }
}
