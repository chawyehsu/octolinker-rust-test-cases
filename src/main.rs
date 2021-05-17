#![allow(dead_code)]
#![allow(unused_imports)]

// Rust 2015, https://doc.rust-lang.org/reference/items/extern-crates.html
extern crate bytes;                                  // @should link to https://docs.rs/bytes
extern crate bytes as my_bytes;                      // @should link to https://docs.rs/bytes
extern crate core as my_core;                        // @should link to https://doc.rust-lang.org/core/
extern crate std as ruststd;                         // @should link to https://doc.rust-lang.org/std/
extern crate lazy_static as _;                       // @should link to https://docs.rs/lazy_static

// Rust 2015,2018, https://doc.rust-lang.org/reference/items/use-declarations.html
// use_item
use ::core;                                          // @should link to https://doc.rust-lang.org/core/
use std;                                             // @should link to https://doc.rust-lang.org/std/
use ::log;                                           // @should link to https://docs.rs/log
use lazy_static;                                     // @should link to https://docs.rs/lazy_static
use ::native_tls as ntls;                            // @should link to https://docs.rs/native-tls
use toml as tjson;                                   // @should link to https://docs.rs/toml
// use_path_segments: https://github.com/rust-analyzer/rust-analyzer/blob/1f1a1ce4f5/crates/syntax/test_data/parser/ok/0010_use_path_segments.rs
use ::core::primitive::i64;                          // @should link to https://doc.rust-lang.org/core/
use std::primitive::u64;                             // @should link to https://doc.rust-lang.org/std/
use ::log::logger;                                   // @should link to https://docs.rs/log
use lazy_static::lazy;                               // @should link to https://docs.rs/lazy_static
use ::mailparse::MailHeader as AMailHeader;          // @should link to https://docs.rs/mailparse
use env_logger::Logger as BLogger;                   // @should link to https://docs.rs/env_logger
// use_path_self_super: https://github.com/rust-analyzer/rust-analyzer/blob/1f1a1ce4f5/crates/syntax/test_data/parser/ok/0013_use_path_self_super.rs
use self::foo::foo_fn;                               // @should link to <CURRENT_PATH>?
mod middle {
    mod inner {
        use super::super::foo_fn;                    // @should link to <???>
    }
}
// use_tree: https://github.com/rust-analyzer/rust-analyzer/blob/1f1a1ce4f5/crates/syntax/test_data/parser/ok/0014_use_tree.rs
use {};                                              // @should not link
use foo::*;                                          // @should link to <???>
use foo::{};                                         // @should link to <???>
use ::std::io::{self, Read as _};                    // @should link to https://doc.rust-lang.org/std/
use std::{io::BufReader, path::{Path, PathBuf}};     // @should link to https://doc.rust-lang.org/std/
use libc::*;                                         // @should link to https://docs.rs/libc
use ::std::{                                         // @should link to https://doc.rust-lang.org/std/
    process::{
        ChildStdin,
        ChildStderr,
    },
    fs::remove_dir as empty_dir
};
use std::{                                           // @should link to https://doc.rust-lang.org/std/
    io::BufWriter,
    path::{
        Component, Components
    }
};
use minimad::{                                       // @should link to https://docs.rs/minimad
    TextTemplate, TextTemplateExpander
};
use {
    async_std::fs::File,                             // @should link to https://docs.rs/async_std
    clap::{                                          // @should link to https://docs.rs/clap
        App, AppSettings, Arg
    },
    chrono::{Utc, DateTime, Date},                   // @should link to https://docs.rs/chrono
    tokio::{                                         // @should link to https://docs.rs/tokio
        net::{
            TcpStream, TcpSocket
        },
        io::BufWriter as TokioWriter,
        io::ErrorKind::{
            Interrupted, InvalidData,
            InvalidInput, AddrInUse
        },
    }
};
// self crate import
use octolinker_rust_test_cases::Error;               // @should link to <???>
// pub use
pub use core::hash::BuildHasher;                     // @should link to https://doc.rust-lang.org/core/
pub use std::io::ErrorKind;                          // @should link to https://doc.rust-lang.org/std/
pub use once_cell;                                   // @should link to https://docs.rs/once_cell
pub use near_sdk::{                                  // @should link to https://docs.rs/near_sdk
    env, ext_contract, near_bindgen, AccountId,
    Balance, Promise, PromiseResult, PublicKey,
};
// pub(self) use, which is the same as leaving it private.
pub(self) use core::slice::SplitMut;                 // @should link to https://doc.rust-lang.org/core/
pub(self) use std::io::Empty;                        // @should link to https://doc.rust-lang.org/std/
pub(self) use terminal_size;                         // @should link to https://docs.rs/terminal_size
pub(self) use near_sdk::{                            // @should link to https://docs.rs/near_sdk
   Duration, BlockHeight
};
// pub(super) use
mod test_pub_super_use {
    pub(super) use core::task::Context;              // @should link to https://doc.rust-lang.org/core/
    pub(super) use std::io::IntoInnerError;          // @should link to https://doc.rust-lang.org/std/
    pub(super) use once_cell;                        // @should link to https://docs.rs/once_cell
    pub(super) use near_sdk::{                       // @should link to https://docs.rs/near_sdk
        env, ext_contract, near_bindgen, AccountId,
        Balance, Promise, PromiseResult, PublicKey,
    };
}
// pub(in path) use
pub(in self) use core::debug_assert_ne;              // @should link to https://doc.rust-lang.org/core/
pub(in self) use std::io::Repeat;                    // @should link to https://doc.rust-lang.org/std/
pub(in self) use serde_json;                         // @should link to https://docs.rs/serde_json
pub(in self) use near_sdk::{                         // @should link to https://docs.rs/near_sdk
    EpochHeight, test_utils, base64, VMConfig
};

// others
mod foo {
    pub(super) fn foo_fn() {}
}
mod inner {
    pub fn hello() {
        use super::ErrorKind;                       // @should link to <???>
        let ek = ErrorKind::NotFound;
        println!("{:?}", ek);
    }
}

fn main() {
    use self::inner;                                // @should link to <CURRENT_PATH>?
    println!("Hello, world!");
    inner::hello();
}

// test modules
#[test]
fn test_my_test() {
    use octolinker_rust_test_cases::Error;          // @should link to <???>
    use crate::inner::hello;                        // @should link to <???>
    hello();
}
