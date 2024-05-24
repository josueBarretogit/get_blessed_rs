use crate::{
    backend::{Categories, CategoriesWithSubCategories, Crates, Table, TableEntry},
    scraper::scraper::{scrape_site, CratesData},
};

#[derive(Debug)]
pub struct ContentParser {
    pub content: CratesData,
}

impl ContentParser {
    pub async fn new() -> Self {
        let page_content = scrape_site().await.unwrap();
        Self {
            content: page_content,
        }
    }

    //Todo! I think hashtables would be useful in this case rather than vectors
    pub fn get_general_crates(&self) -> Table {
        let general_table : Table = Table {
            entries: [
                TableEntry {
                    use_case: "".to_string(),
                    crates: [
                        Crates {
                            name: "rand".into(),
                            description: "De facto standard random number generation library split out from the standard library,".into(),
                            features : None
                        },
                    ].to_vec(),
                },
                TableEntry {
                    use_case: "".to_string(),
                    crates: [
                        Crates {
                            name: "time".into(),
                            description: "A smaller, simpler library. Preferrable if covers your needs, but it's quite limited in what it provides.,".into(),
                            features : Some(vec!["macros ".into(), "formatting ".into(), "parsing ".into()])
                        },
                        Crates {
                            name: "chrono".into(),
                            description: "The most comprehensive and full-featured datetime library, but more complex because of it.,".into(),
                            features: Some(vec!["serde".into()]),
                        },
                    ].to_vec(),
                },
                TableEntry {
                    use_case: "".to_string(),
                    crates: [
                        Crates {
                            name: "serde".into(),
                            description: "De facto standard serialization library. Use in conjunction with sub-crates like serde_json for the specific format that you are using.,".into(),
                            features : Some(vec!["derive".into()])
                        },
                    ].to_vec(),
                },
                TableEntry {
                    use_case: "".to_string(),
                    crates: [
                        Crates {
                            name: "regex".into(),
                            description: "De facto standard regex library. Very fast, but does not support fancier features such as backtracking.,".into(),
                            features : None
                        },
                        Crates {
                            name: "fancy-regex".into(),
                            description: "Use if need features such as backtracking which regex doesn't support,".into(),
                            features : None
                        },
                    ].to_vec(),
                },
                TableEntry {
                    use_case: "".into(),
                    crates: [
                        Crates {
                            name: "uuid".into(),
                            description: "Implements generating and parsing UUIDs and a number of utility functions,".into(),
                            features : Some(vec!["v4 ".into(), "serde ".into()])
                        },
                    ].to_vec(),
                },
                TableEntry {
                    use_case: "".into(),
                    crates: [
                        Crates {
                            name: "tempfile".into(),
                            description: "Supports both temporary files and temporary directories,".into(),
                            features : None
                        },
                    ].to_vec(),
                },
                TableEntry {
                    use_case: "".into(),
                    crates: [
                        Crates {
                            name: "flate2".into(),
                            description: "Uses a pure-Rust implementation by default. Use feature flags to opt in to system zlib.,".into(),
                            features : None
                        },
                    ].to_vec(),
                },
                TableEntry {
                    use_case: "".into(),
                    crates: [
                        Crates {
                            name: "indexmap".into(),
                            description: "A HashMap that seperately keeps track of insertion order and allows you to efficiently iterate over its elements in that order,".into(),
                            features : None
                        },
                    ].to_vec(),
                },
                TableEntry {
                    use_case: "".into(),
                    crates: [
                        Crates {
                            name: "arrayvec".into(),
                            description: "Arrays that are ONLY stack-allocated with fixed capacity,".into(),
                            features : Some(vec!["serde".into()])
                        },
                        Crates {
                            name: "smallvec".into(),
                            description: "Arrays that are stack-allocated with fallback to the heap if the fixed stack capacity is exceeded,".into(),
                            features : None,
                        },
                        Crates {
                            name: "tinyvec".into(),
                            description: "Stack allocated arrays in 100% safe Rust code but requires items to implement the Default trait.,".into(),
                            features : None
                        },
                    ].to_vec(),
                },
                TableEntry {
                    use_case: "".into(),
                    crates: [
                        Crates {
                            name: "reqwest".into(),
                            description: "Full-fat HTTP client. Can be used in both synchronous and asynchronous code. Requires tokio runtime.,".into(),
                            features : Some(vec!["json".into()])
                        },
                        Crates {
                            name: "ureq".into(),
                            description: "Minimal synchronous HTTP client focussed on simplicity and minimising dependencies.,".into(),
                            features : Some(vec!["json ".into(), "charset ".into()])
                        },
                    ].to_vec(),
                },
            ].to_vec(),
        };
        general_table
    }

    pub fn get_crates(&self, category: Categories) -> Table {
        let math_crates = Table {
            entries: [
                TableEntry {
                    use_case: "".into(),
                    crates: [
                        Crates {
                            name: "num-traits".into(),
                            description: "Traits like Number, Add, etc that allow you write functions that are generic over the specific numeric type,".into(),
                            features : None
                        },
                    ].to_vec(),
                },
                TableEntry {
                    use_case: "".into(),
                    crates: [
                        Crates {
                            name: "num-bigint".into(),
                            description: "It's not the fastest, but it's part of the trusted num library.,".into(),
                            features : Some(vec!["rand".into()])
                        },
                        Crates {
                            name: "rug".into(),
                            description: "LGPL licensed. Wrapper for GMP. Much faster than num-bigint.,".into(),
                            features : None
                        },
                    ].to_vec(),
                },
                TableEntry {
                    use_case: "".into(),
                    crates: [
                        Crates {
                            name: "rust_decimal".into(),
                            description: "The binary representation consists of a 96 bit integer number, a scaling factor used to specify the decimal fraction and a 1 bit sign.,".into(),
                            features : None
                        },
                    ].to_vec(),
                },
                TableEntry {
                    use_case: "".into(),
                    crates: [
                        Crates {
                            name: "ordered-float".into(),
                            description: "Float types that don't allow NaN and are therefore orderable. You can also use the,total_cmp,method from the standard library like,.sort_by(|a, b| a.total_cmp(&b)),.,".into(),
                            features : None
                        },
                    ].to_vec(),
                },
                TableEntry {
                    use_case: "".into(),
                    crates: [
                        Crates {
                            name: "nalgebra".into(),
                            description: "General-purpose linear algebra library with transformations and statically-sized or dynamically-sized matrices. However it supports only vectors (1d) and matrices (2d) and not higher-dimensional tensors.,".into(),
                            features : Some(vec!["serde_serialize".into()])
                        },
                        Crates {
                            name: "ndarray".into(),
                            description: "Less featureful than nalgebra but supports arbitrarily dimensioned arrays,".into(),
                            features : None
                        },
                    ].to_vec(),
                },
                TableEntry {
                    use_case: "".into(),
                    crates: [
                        Crates {
                            name: "polars".into(),
                            description: "Similar to the Pandas library in Python but in pure Rust. Uses the Apache Arrow Columnar Format as the memory model.,".into(),
                            features : None
                        },
                        Crates {
                            name: "datafusion".into(),
                            description: "Apache DataFusion,is an in-memory query engine that uses Apache Arrow as the memory model,".into(),
                            features : None
                        },
                    ].to_vec(),
                },
            ].to_vec(),
        };

        let ff_crates =  Table {
            entries: [
                TableEntry {
                    use_case: "".into(),
                    crates: [
                        Crates {
                            name: "bindgen".into(),
                            description: "Generate Rust bindings to C libraries,".into(),
                            features : None
                        },
                        Crates {
                            name: "cbindgen".into(),
                            description: "Generate C bindings to Rust libraries,".into(),
                            features : None
                        },
                    ].to_vec(),
                },
                TableEntry {
                    use_case: "".into(),
                    crates: [
                        Crates {
                            name: "cxx".into(),
                            description: "Safe C++ <-> Rust interop by generating code for both sides.,".into(),
                            features : None
                        },
                    ].to_vec(),
                },
                TableEntry {
                    use_case: "".into(),
                    crates: [
                        Crates {
                            name: "pyo3".into(),
                            description: "Supports both calling python code from Rust and exposing Rust code to Python,".into(),
                            features : Some(vec!["extension-module".into()])
                        },
                    ].to_vec(),
                },
                TableEntry {
                    use_case: "".into(),
                    crates: [
                        Crates {
                            name: "napi".into(),
                            description: "is a framework for building pre-compiled Node.js addons in Rust.,".into(),
                            features : None,
                        },
                        Crates {
                            name: "neon".into(),
                            description: "Slower than napi, but also widely used and well-maintained,".into(),
                            features : None
                        },
                    ].to_vec(),
                },
                TableEntry {
                    use_case: "".into(),
                    crates: [
                        Crates {
                            name: "rutie".into(),
                            description: "Supports both embedding Rust into Ruby applications and embedding Ruby into Rust applications,".into(),
                            features : None
                        },
                    ].to_vec(),
                },
                TableEntry {
                    use_case: "".into(),
                    crates: [
                        Crates {
                            name: "objc".into(),
                            description: "Interop with the Objective-C runtime,".into(),
                            features : None
                        },
                    ].to_vec(),
                },
                TableEntry {
                    use_case: "".into(),
                    crates: [
                        Crates {
                            name: "jni".into(),
                            description: "Implement Java methods for JVM and Android in Rust. Call Java code from Rust. Embed JVM in Rust applications.,".into(),
                            features : None
                        },
                    ].to_vec(),
                },
                TableEntry {
                    use_case: "".into(),
                    crates: [
                        Crates {
                            name: "mlua".into(),
                            description: "Bindings to Lua 5.4, 5.3, 5.2, 5.1 (including LuaJIT),".into(),
                            features : Some(vec!["lua54 ".into(), "vendored ".into()])
                        },
                    ].to_vec(),
                },
                TableEntry {
                    use_case: "".into(),
                    crates: [
                        Crates {
                            name: "flutter_rust_bridge".into(),
                            description: "Works with Dart with or without Flutter,".into(),
                            features : None
                        },
                    ].to_vec(),
                },
                TableEntry {
                    use_case: "".into(),
                    crates: [
                        Crates {
                            name: "rustler".into(),
                            description: "Safe Rust bridge for creating Erlang NIF functions,".into(),
                            features : Some(vec!["nif_version_2_16".into()])
                        },
                    ].to_vec(),
                },
            ].to_vec(),
        };

        let cryptography_crates = Table {
    entries: [
        TableEntry {
            use_case: "".into(),
            crates: [
                Crates {
                    name: "argon2".into(),
                    description: "no description".into(),
                    features : Some(vec!["rand".into()])
                },
                Crates {
                    name: "scrypt".into(),
                    description: "no description".into(),
                    features : Some(vec!["password-hash".into()])
                },
                Crates {
                    name: "bcrypt".into(),
                    description: "no description".into(),
                            features : None
                },
            ].to_vec(),
        },
        TableEntry {
            use_case: "".into(),
            crates: [
                Crates {
                    name: "sha2".into(),
                    description: "no description".into(),
                            features : None
                },
                Crates {
                    name: "sha1".into(),
                    description: "no description".into(),
                            features : None
                },
                Crates {
                    name: "md-5".into(),
                    description: "no description".into(),
                            features : None
                },
            ].to_vec(),
        },
        TableEntry {
            use_case: "".into(),
            crates: [
                Crates {
                    name: "aes-gcm-siv".into(),
                    description: "no description".into(),
                            features : None
                },
                Crates {
                    name: "aes-gcm".into(),
                    description: "no description".into(),
                            features : None
                },
                Crates {
                    name: "chacha20poly1305".into(),
                    description: "no description".into(),
                            features : None
                },
            ].to_vec(),
        },
        TableEntry {
            use_case: "".into(),
            crates: [
                Crates {
                    name: "rsa".into(),
                    description: "no description".into(),
                            features : None
                },
            ].to_vec(),
        },
        TableEntry {
            use_case: "".into(),
            crates: [
                Crates {
                    name: "ed25519".into(),
                    description: "Use in conjunction with the ed25519-dalek crate.".into(),
                            features : None
                },
                Crates {
                    name: "ecdsa".into(),
                    description: "no description".into(),
                            features : None
                },
                Crates {
                    name: "dsa".into(),
                    description: "no description".into(),
                            features : None
                },
            ].to_vec(),
        },
        TableEntry {
            use_case: "".into(),
            crates: [
                Crates {
                    name: "der".into(),
                    description: "no description".into(),
                            features : None
                },
                Crates {
                    name: "pem-rfc7468".into(),
                    description: "no description".into(),
                            features : None
                },
                Crates {
                    name: "pkcs8".into(),
                    description: "no description".into(),
                            features : None
                },
                Crates {
                    name: "x509-cert".into(),
                    description: "no description".into(),
                            features : None
                },
            ].to_vec(),
        },
        TableEntry {
            use_case: "".into(),
            crates: [
                Crates {
                    name: "rustls".into(),
                    description: "A portable pure-rust high-level implementation of TLS. Implements TLS 1.2 and higher.".into(),
                            features : None
                },
                Crates {
                    name: "native-tls".into(),
                    description: "Delegates to the system TLS implementations on windows and macOS, and uses OpenSSL on linux.".into(),
                            features : None
                },
                Crates {
                    name: "webpki".into(),
                    description: "X.509 Certificate validation. Builds on top of ring.".into(),
                            features : None
                },
                Crates {
                    name: "ring".into(),
                    description: "Fork of BoringSSL. Provides low-level cryptographic primitives for TLS/SSL.".into(),
                            features : None
                },
            ].to_vec(),
        },
        TableEntry {
            use_case: "".into(),
            crates: [
                Crates {
                    name: "subtle".into(),
                    description: "Utilities for writing constant-time algorithms.".into(),
                            features : None
                },
                Crates {
                    name: "zeroize".into(),
                    description: "Securely erase memory.".into(),
                            features : None
                },
            ].to_vec(),
        },
    ].to_vec(),
};

        match category {
            Categories::Math => math_crates,
            Categories::FFI => ff_crates,
            Categories::Cryptography => cryptography_crates,
        }
    }

    pub fn get_crates_with_sub(&self, category: CategoriesWithSubCategories) -> Table {
        let cli_crates = Table {
    entries: [
        TableEntry {
            use_case: "".into(),
            crates: [
                Crates {
                    name: "clap".into(),
                    description: "Ergonomic, battle-tested, includes the kitchen sink, and is fast at runtime. However compile times can be slow".into(),
                            features : Some(vec!["derive".into()])
                },
                Crates {
                    name: "lexopt".into(),
                    description: "Fast compile times, fast runtime, pedantic about correctness. API is less ergonomic".into(),
                            features : None
                },
                Crates {
                    name: "pico-args".into(),
                    description: "Fast compile times, fast runtime, more lax about correctness. API is more ergonomic".into(),
                            features : None
                },
            ].to_vec(),
        },
        TableEntry {
            use_case: "".into(),
            crates: [
                Crates {
                    name: "globset".into(),
                    description: "High-performance globbing that allows multiple globs to be evaluated at once".into(),
                            features : None
                },
                Crates {
                    name: "walkdir".into(),
                    description: "Basic recursive filesystem walking.".into(),
                            features : None
                },
                Crates {
                    name: "ignore".into(),
                    description: "Recursive filesystem walking that respects ignore files (like .gitignore)".into(),
                            features : None
                },
                Crates {
                    name: "notify".into(),
                    description: "Watch files or directories and execute a function when they change".into(),
                            features : Some(vec!["serde".into()])
                },
            ].to_vec(),
        },
        TableEntry {
            use_case: "".into(),
            crates: [
                Crates {
                    name: "termcolor".into(),
                    description: "Cross-platform terminal colour output".into(),
                            features : None
                },
                Crates {
                    name: "indicatif".into(),
                    description: "Progress bars and spinners".into(),
                            features : None
                },
                Crates {
                    name: "ratatui".into(),
                    description: "A high-level TUI library with widgets, layout, etc.".into(),
                            features : Some(vec!["all-widgets".into()])
                },
                Crates {
                    name: "crossterm".into(),
                    description: "Low-level cross-platform terminal rendering and event handling".into(),
                            features : Some(vec!["event-stream".into()])
                },
                Crates {
                    name: "inquire".into(),
                    description: "Ask for confirmation, selection, text input and more".into(),
                            features : None
                },
            ].to_vec(),
        },
    ].to_vec(),
};

        let common_crates = Table {
    entries: [
        TableEntry {
            use_case: "".into(),
            crates: [
                Crates {
                    name: "anyhow".into(),
                    description: "Provides a boxed error type that can hold any error, and helpers for generating an application-level stack trace.".into(),
                            features : None
                },
                Crates {
                    name: "color-eyre".into(),
                    description: "A fork of anyhow that gives you more control over the format of the generated error messages. Recommended if you intend to present error messages to end users. Otherwise anyhow is simpler.".into(),
                            features : None
                },
                Crates {
                    name: "thiserror".into(),
                    description: "Helps with generating boilerplate for enum-style error types.".into(),
                            features : None
                },
            ].to_vec(),
        },
        TableEntry {
            use_case: "".into(),
            crates: [
                Crates {
                    name: "tracing".into(),
                    description: "Tracing is now the go-to crate for logging.".into(),
                            features : None
                },
                Crates {
                    name: "log".into(),
                    description: "An older and simpler crate if your needs are simple and you are not using any async code.".into(),
                            features : None
                },
                Crates {
                    name: "tracing".into(),
                    description: "Tracing is now the go-to crate for logging.".into(),
                            features : None
                },
                Crates {
                    name: "slog".into(),
                    description: "Structured logging".into(),
                            features : Some(vec!["max_level_trace ".into(), "release_max_level_warn ".into()])
                },
            ].to_vec(),
        },
        TableEntry {
            use_case: "".into(),
            crates: [
                Crates {
                    name: "once_cell".into(),
                    description: "Newer crate with more ergonomic API. Should be preferred for all new projects.".into(),
                            features : None
                },
                Crates {
                    name: "lazy_static".into(),
                    description: "Older crate. API is less convenient, but crate is stable and maintained.".into(),
                            features : None
                },
                Crates {
                    name: "itertools".into(),
                    description: "A bunch of useful methods on iterators that aren't in the stdlib".into(),
                            features : None
                },
                Crates {
                    name: "syn".into(),
                    description: "Parse rust source code".into(),
                            features : Some(vec!["full".into()])
                },
                Crates {
                    name: "quote".into(),
                    description: "Quasi quoting rust (useful for interpolating generated code with literal code)".into(),
                            features : None
                },
                Crates {
                    name: "paste".into(),
                    description: "Concatenating and manipulating identifiers".into(),
                            features : None
                },
                Crates {
                    name: "bytemuck".into(),
                    description: "no description".into(),
                            features : Some(vec!["derive".into()])
                },
                Crates {
                    name: "zerocopy".into(),
                    description: "no description".into(),
                            features : None
                },
                Crates {
                    name: "bitflags".into(),
                    description: "Strongly typed bitflag types".into(),
                            features : None
                },
            ].to_vec(),
        },
        TableEntry {
            use_case: "".into(),
            crates: [
                Crates {
                    name: "memmap2".into(),
                    description: "The older memmap crate is unmaintained.".into(),
                            features : None
                },
                Crates {
                    name: "libc".into(),
                    description: "Bindings for directly calling libc functions.".into(),
                            features : None
                },
                Crates {
                    name: "windows".into(),
                    description: "The official Microsoft-provided crate for interacting with windows APIs".into(),
                            features : Some(vec![

"Data_Xml_Dom".into(),
    "Win32_Foundation".into(),
    "Win32_Security".into(),
    "Win32_System_Threading".into(),
    "Win32_UI_WindowsAndMessaging".into(),


                            ])
                },
                Crates {
                    name: "winapi".into(),
                    description: "Older binding to the windows APIs. Unofficial, but more complete than windows-rs".into(),
                            features : None
                },
                Crates {
                    name: "nix".into(),
                    description: "Bindings to the various *nix system functions. (Unix, Linux, MacOS, etc.)".into(),
                            features : None
                },
            ].to_vec(),
        },
    ].to_vec(),
};

        let graphics_crates =  Table {
    entries: [
        TableEntry {
            use_case: "".into(),
            crates: [
                Crates {
                    name: "gtk4".into(),
                    description: "Rust bindings to GTK4. These are quite well supported, although you'll often need to use the C documentation.".into(),
                            features : Some(vec!["v4_2".into()])
                },
                Crates {
                    name: "relm4".into(),
                    description: "A higher-level library that sits on top of gtk4-rs".into(),
                            features : None
                },
                Crates {
                    name: "tauri".into(),
                    description: "Electron-like web-based UI. Except it uses system webviews rather than shipping chromium, and non-UI code is written in Rust rather than node.js".into(),
                            features : None
                },
                Crates {
                    name: "dioxus".into(),
                    description: "A very nice API layer that has Tauri, Web, and TUI renderers. A native renderer is coming soon.".into(),
                            features : Some(vec!["fullstack".into()])
                },
                Crates {
                    name: "egui".into(),
                    description: "Immediate-mode UI. Lots of widgets. The most useable out of the box if your needs are simple and you don't need to customise of the look and feel".into(),
                            features : None
                },
                Crates {
                    name: "iced".into(),
                    description: "Retained mode UI with a nice API. It's useable for basic apps, but has a number of missing features including multiple windows, layers, and proper text rendering.".into(),
                            features : None
                },
                Crates {
                    name: "floem".into(),
                    description: "Inspired by Xilem, Leptos and rui, floem is currently more complete than any of them for native UI. Used by the Lapce text editor.".into(),
                            features : None
                },
                Crates {
                    name: "vizia".into(),
                    description: "Fairly complete with sophisticated layout and text layout, but has yet to make a stable release.".into(),
                            features : None
                },
                Crates {
                    name: "winit".into(),
                    description: "The defacto standard option. Uses an event loop based architecture. Widely used and should probably be the default choice.".into(),
                            features : None
                },
                Crates {
                    name: "tao".into(),
                    description: "A fork of winit by the Tauri project which adds support for things like system menus that desktop apps need.".into(),
                            features : None
                },
                Crates {
                    name: "glazier".into(),
                    description: "A new competitor to winit based on the old druid-shell. Has a callback that may be better than the event loop architecture for some tasks. Doesn't yet have a stable release.".into(),
                            features : None
                },
                Crates {
                    name: "baseview".into(),
                    description: "Specialized window creation library targetting windows to be embedded in other applications (e.g. DAW plugins)".into(),
                            features : None
                },
                Crates {
                    name: "femtovg".into(),
                    description: "OpenGL based. Offers a simple API. Probably the easiest to get started with.".into(),
                            features : Some(vec!["serde".into()])
                },
                Crates {
                    name: "skia-safe".into(),
                    description: "Bindings to the Skia C++ library. The most complete option with excellent performance. However, it can be difficult to get it to compile.".into(),
                            features : None
                },
                Crates {
                    name: "vello".into(),
                    description: "WGPU based and uses cutting edge techniques to render vector paths using the GPU. Still somewhat immature and hasn't yet put out a stable release.".into(),
                            features : None
                },
                Crates {
                    name: "vger".into(),
                    description: "A simpler WGPU based option which is less innovative but currently more stable than vello.".into(),
                            features : None
                },
                Crates {
                    name: "webrender".into(),
                    description: "OpenGL based. Mature with production usage in Firefox but documentation and OSS maintenance are lacking.".into(),
                            features : None
                },
                Crates {
                    name: "taffy".into(),
                    description: "Supports Flexbox and CSS Grid algorithms.".into(),
                            features : Some(vec!["serde".into(), "flexbox".into()])
                },
                Crates {
                    name: "morphorm".into(),
                    description: "Implements it's own layout algorithm based on Subform layout".into(),
                            features : None
                },
                Crates {
                    name: "cosmic-text".into(),
                    description: "Full text layout including rich text and support for BiDi and non-latin scripts. The best option for now.".into(),
                            features : None
                },
                Crates {
                    name: "parley".into(),
                    description: "Another very accomplished text layout library used by Druid/Xilem.".into(),
                            features : None
                },
                Crates {
                    name: "accesskit".into(),
                    description: "Allows you to export a semantic tree representing your UI to make accessible to screen readers and other assistive technologies".into(),
                            features : None
                },
                Crates {
                    name: "arboard".into(),
                    description: "A fork of rust-clipboard that supports copy and pasting of both text and images on Linux (X11/Wayland), MacOS and Windows.".into(),
                            features : None
                },
                Crates {
                    name: "rfd".into(),
                    description: "Platform-native open/save file dialogs. Can be used in conjunction with other UI libraries.".into(),
                            features : None
                },
            ].to_vec(),
        },
        TableEntry {
            use_case: "".into(),
            crates: [
                Crates {
                    name: "bevy".into(),
                    description: "An ECS based game engine, good for 3D but also capable of 2D.".into(),
                            features : None
                },
                Crates {
                    name: "fyrox".into(),
                    description: "An OOP-focused game engine with 3D and 2D support and a full GUI scene editor.".into(),
                            features : None
                },
                Crates {
                    name: "ggez".into(),
                    description: "A simpler option for 2d games only.".into(),
                            features : None
                },
                Crates {
                    name: "macroquad".into(),
                    description: "A simple and easy to use 2d game library, great for beginners.".into(),
                            features : None
                },
                Crates {
                    name: "glam".into(),
                    description: "Fast math library optimised for game development use cases".into(),
                            features : None
                },
            ].to_vec(),
        },
    ].to_vec(),
};

        let database_crates = Table {
    entries: [
        TableEntry {
            use_case: "".into(),
            crates: [
                Crates {
                    name: "sqlx".into(),
                    description: "Works with Postgres, MySQL, SQLite, and MS SQL.Supports compile time checking of queries. Async: supports both tokio and async-std.".into(),
                            features : Some(vec!["runtime-tokio".into(), "tls-native-tls".into(), "macros".into(), "chrono".into(), "mysql".into()])
                },
                Crates {
                    name: "diesel".into(),
                    description: "Has excellent performance and takes an approach of strict compile time guarantees. The main crate is Sync only, but diesel-async provides an async connection implementation.".into(),
                            features : Some(vec!["mysql".into()])
                },
                Crates {
                    name: "sea-orm".into(),
                    description: "Built on top of sqlx (see above). There is also a related sea-query crate that provides a query builder without full ORM functionality.".into(),
                            features : None
                },
                Crates {
                    name: "tokio-postgres".into(),
                    description: "Postgres-specific library. Performs better than SQLx".into(),
                            features : None
                },
                Crates {
                    name: "mysql_async".into(),
                    description: "Has a poorly designed API. Prefer SQLx or Diesel for MySQL".into(),
                            features : Some(vec!["derive".into()])
                },
                Crates {
                    name: "rusqlite".into(),
                    description: "Provides a sync API to SQLite + provides access to advanced sqlite features.".into(),
                            features : Some(vec!["bundled".into()])
                },
                Crates {
                    name: "tiberius".into(),
                    description: "MS SQL specific library. Has better support for advanced column types than SQLx.".into(),
                            features : Some(vec!["chrono".into(), "rustls".into()])
                },
                Crates {
                    name: "diesel-oci".into(),
                    description: "Diesel backend and connection implementation for oracle databases".into(),
                            features : None
                },
                Crates {
                    name: "oracle".into(),
                    description: "Rust bindings to ODPI-C".into(),
                            features : Some(vec!["chrono".into(), ])
                },
            ].to_vec(),
        },
        TableEntry {
            use_case: "".into(),
            crates: [
                Crates {
                    name: "redis".into(),
                    description: "no description ".into(),
                            features : None
                },
                Crates {
                    name: "mongodb".into(),
                    description: "no description ".into(),
                            features : Some(vec!["async-std-runtime".into()])
                },
                Crates {
                    name: "elasticsearch".into(),
                    description: "no description ".into(),
                            features : None
                },
                Crates {
                    name: "rocksdb".into(),
                    description: "no description ".into(),
                            features : None
                },
                Crates {
                    name: "cassandra-protocol".into(),
                    description: "Low-level Cassandra protocol implementation.".into(),
                            features : None
                },
                Crates {
                    name: "cdrs-tokio".into(),
                    description: "High-level async Cassandra driver.".into(),
                            features : None
                },
            ].to_vec(),
        },
        TableEntry {
            use_case: "".into(),
            crates: [
                Crates {
                    name: "deadpool".into(),
                    description: "A dead simple async pool for connections and objects of any type.".into(),
                            features : Some(vec!["serde".into()])
                },
            ].to_vec(),
        },
    ].to_vec(),
};

        let networking_crates =  Table {
    entries: [
        TableEntry {
            use_case: "".into(),
            crates: [
                Crates {
                    name: "tokio".into(),
                    description: "The oldest async runtime in the Rust ecosystem and still the most widely supported. Recommended for new projects.".into(),
                            features : Some(vec!["full".into()])
                },
                Crates {
                    name: "futures-executor".into(),
                    description: "A minimal executor. In particular, the block_on function is useful if you want to run an async function synchronously in codebase that is mostly synchronous.".into(),
                            features : None
                },
                Crates {
                    name: "futures".into(),
                    description: "Utility functions for working with Futures and Streams".into(),
                            features : None
                },
                Crates {
                    name: "async-trait".into(),
                    description: "Provides a workaround for the lack of language support for async functions in traits".into(),
                            features : None
                },
                Crates {
                    name: "glommio".into(),
                    description: "Use if you need io_uring support. Still somewhat experimental but rapidly maturing.".into(),
                            features : None
                },
            ].to_vec(),
        },
        TableEntry {
            use_case: "".into(),
            crates: [
                Crates {
                    name: "http".into(),
                    description: "The `http` crate doesn't actually contain an HTTP implementation. Just types and interfaces to help interoperability.".into(),
                            features : None
                },
                Crates {
                    name: "hyper".into(),
                    description: "A low-level HTTP implementation (both client and server). Implements HTTP/1, and HTTP/2. Works best with the tokio async runtime, but can support other runtimes.".into(),
                            features : None
                },
                Crates {
                    name: "reqwest".into(),
                    description: "Full-fat HTTP client. Can be used in both synchronous and asynchronous code. Requires tokio runtime.".into(),
                            features : Some(vec!["json".into()])
                },
                Crates {
                    name: "ureq".into(),
                    description: "Minimal synchronous HTTP client focussed on simplicity and minimising dependencies.".into(),
                            features : Some(vec!["json".into()])
                },
                Crates {
                    name: "axum".into(),
                    description: "A minimal and ergonomic framework. An official part of the tokio project. Recommend for most new projects.".into(),
                            features : Some(vec!["multipart".into(),])
                },
                Crates {
                    name: "actix-web".into(),
                    description: "A performance focussed framework. All Rust frameworks are fast, but choose actix-web if you need the absolutely maximum performance.".into(),
                            features : None
                },
                Crates {
                    name: "async-graphql".into(),
                    description: "A high-performance graphql server library that's fully specification compliant. Integrates with actix-web, axum, poem, rocket, tide, warp.".into(),
                            features : Some(vec!["chrono".into()])
                },
            ].to_vec(),
        },
        TableEntry {
            use_case: "".into(),
            crates: [
                Crates {
                    name: "tungstenite".into(),
                    description: "Low-level crate that others build on".into(),
                            features : None
                },
                Crates {
                    name: "tokio-tungstenite".into(),
                    description: "If you are using the tokio executor".into(),
                            features : None
                },
                Crates {
                    name: "async-tungstenite".into(),
                    description: "If you are using the async-std executor".into(),
                            features : None
                },
            ].to_vec(),
        },
        TableEntry {
            use_case: "".into(),
            crates: [
                Crates {
                    name: "tonic".into(),
                    description: "gRPC over HTTP/2 with full support for asynchronous code. Works with tokio".into(),
                            features : None
                },
            ].to_vec(),
        },
    ].to_vec(),
};

        let concurrency_crates = Table {
    entries: [
        TableEntry {
            use_case: "".into(),
            crates: [
                Crates {
                    name: "parking_lot".into(),
                    description: "std::sync::Mutex also works fine. But Parking Lot is faster.".into(),
                            features : None
                },
                Crates {
                    name: "arc-swap".into(),
                    description: "Useful for sharing data that has many readers but few writers".into(),
                            features : None
                },
                Crates {
                    name: "dashmap".into(),
                    description: "The fastest for general purpose workloads".into(),
                            features : Some(vec!["serde ".into(), "rayon ".into()])
                },
                Crates {
                    name: "flurry".into(),
                    description: "Particularly good for read-heavy workloads.".into(),
                            features : None
                },
                Crates {
                    name: "crossbeam-channel".into(),
                    description: "The absolute fastest channel implementation available. Implements Go-like 'select' feature.".into(),
                            features : None
                },
                Crates {
                    name: "flume".into(),
                    description: "Smaller and simpler than crossbeam-channel and almost as fast".into(),
                            features : None
                },
                Crates {
                    name: "tokio".into(),
                    description: "Tokio's sync module provides channels for using in async code".into(),
                            features : Some(vec!["full".into()])
                },
                Crates {
                    name: "postage".into(),
                    description: "Channels that integrate nicely with async code, with different options than Tokio".into(),
                            features : None
                },
                Crates {
                    name: "rayon".into(),
                    description: "Convert sequential computation into parallel computation with one call - `par_iter` instead of `iter`".into(),
                            features : None
                },
            ].to_vec(),
        },
    ].to_vec(),
};

        match category {
            CategoriesWithSubCategories::Clis => cli_crates,
            CategoriesWithSubCategories::Common => common_crates,
            CategoriesWithSubCategories::Graphics => graphics_crates,
            CategoriesWithSubCategories::Databases => database_crates,
            CategoriesWithSubCategories::Networking => networking_crates,
            CategoriesWithSubCategories::Concurrency => concurrency_crates,
        }
    }
}
