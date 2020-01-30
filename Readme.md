# jreflection

[![Crates.io](https://img.shields.io/crates/v/jreflection.svg)](https://crates.io/crates/jreflection)
[![Docs](https://docs.rs/jreflection/badge.svg)](https://docs.rs/jreflection/)
[![GitHub](https://img.shields.io/github/stars/MaulingMonkey/jreflection.svg?label=GitHub&style=social)](https://github.com/MaulingMonkey/jreflection)
[![unsafe: yes](https://img.shields.io/github/search/MaulingMonkey/jreflection/unsafe%2bextension%3Ars?color=yellow&label=unsafe)](https://github.com/MaulingMonkey/jreflection/search?q=unsafe+extension%3Ars)
[![rust: 1.36.0+](https://img.shields.io/badge/rust-1.36.0%2B-green.svg)](https://gist.github.com/MaulingMonkey/c81a9f18811079f19326dac4daa5a359#minimum-supported-rust-versions-msrv)
[![License](https://img.shields.io/crates/l/jreflection.svg)](https://github.com/MaulingMonkey/jreflection)
[![dependency status](https://deps.rs/repo/github/MaulingMonkey/jreflection/status.svg)](https://deps.rs/repo/github/MaulingMonkey/jreflection)

**J**ava **Reflection**

Static reflection APIs for analyzing jars and jimage files.

This crate mainly exists to support [jni-bindgen](https://github.com/MaulingMonkey/jni-bindgen).
However, it attempts to support fully parsing jars and jimages, so it has other potential uses.
In particular, I'm getting *very* tempted to use it for some form of multi-JDK documentation generation.

| Branch | Badges | Notes |
| ------ | ------ | ----- |
| [publish](https://github.com/MaulingMonkey/jreflection/tree/publish)  | [![Open issues](https://img.shields.io/github/issues-raw/MaulingMonkey/jreflection.svg)](https://github.com/MaulingMonkey/jreflection/issues) | Stable/published version
| [master](https://github.com/MaulingMonkey/jreflection/tree/master)    | [![Build Status](https://travis-ci.org/MaulingMonkey/jreflection.svg)](https://travis-ci.org/MaulingMonkey/jreflection) | "Completed" stuff that hasn't been published.
| wip/*                                                                 | | "Work In Progress" - incomplete, use at your own risk.
| dead/*                                                                | | Abandoned threads of work

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

<!-- https://doc.rust-lang.org/1.4.0/complement-project-faq.html#why-dual-mit/asl2-license? -->
<!-- https://rust-lang-nursery.github.io/api-guidelines/necessities.html#crate-and-its-dependencies-have-a-permissive-license-c-permissive -->
<!-- https://choosealicense.com/licenses/apache-2.0/ -->
<!-- https://choosealicense.com/licenses/mit/ -->
