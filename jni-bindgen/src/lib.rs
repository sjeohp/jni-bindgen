//! Parse .jar s and  .class es to generate Rust FFI bindings using JNI.

#[macro_use] mod java;

/// Configuration formats for invoking jni_bindgen
pub mod config { // Part of the actual official API of this crate.
    #[allow(unused_imports)] use super::*;

    pub mod runtime;
    pub mod toml;
}

/// Rust generation logic
mod emit_rust {
    #[allow(unused_imports)] use super::*;

    mod context;
    mod fields;
    mod known_docs_url;
    mod methods;
    mod modules;
    mod preamble;
    mod structs;

    pub use context::Context;
    use fields::*;
    use known_docs_url::*;
    use methods::*;
    use modules::*;
    use preamble::*;
    use structs::*;
}

/// JNI and Rust identifier parsing and categorizing utilities
mod identifiers {
    #[allow(unused_imports)] use super::*;
    use std::iter::*;

    mod field_mangling_style;
    mod method_mangling_style;
    mod rust_identifier;

    pub use field_mangling_style::*;
    pub use method_mangling_style::*;
    pub use rust_identifier::*;
}

/// Core generation logic
mod run {
    #[allow(unused_imports)] use super::*;

    mod run;

    pub use run::run;
    pub use run::RunResult;
}

mod util {
    #[allow(unused_imports)] use super::*;

    mod dedupe_file_set;
    mod difference;
    mod generated_file;
    mod progress;

    pub use dedupe_file_set::{ConcurrentDedupeFileSet, DedupeFileSet};
    pub use difference::Difference;
    pub use generated_file::write_generated;
    pub use progress::Progress;
}


pub(crate) use identifiers::*;
pub use run::run;
pub use run::RunResult;
