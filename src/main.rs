#![allow(dead_code)]
#![allow(unused_imports)]

// Rust 2015, https://doc.rust-lang.org/reference/items/extern-crates.html
extern crate bytes;
extern crate bytes as my_bytes;
extern crate core as my_core;
extern crate std as ruststd;
extern crate lazy_static as _;

// Rust 2018, https://doc.rust-lang.org/reference/items/use-declarations.html
use ::core::primitive::i64;
use ::std::process::Command;
use ::std::{
    process::{
        ChildStdin,
        ChildStderr,
    },
    fs::remove_dir as empty_dir
};
use core::primitive::u8;
use std;
use std::str::from_utf8 as _my_from_utf8;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::io::{self, Read as _};
use std::{io::BufReader, path::{Path, PathBuf}};
use std::{
    io::BufWriter,
    path::{
        Component, Components
    }
};
use lazy_static::lazy;
use {
    async_std::fs::File,
    clap::{
        App, AppSettings, Arg
    },
    chrono::{Utc, DateTime, Date},
    tokio::{
        net::{
            TcpStream, TcpSocket
        },
        io::BufWriter as TokioWriter,
        io::ErrorKind::{
            Interrupted, InvalidData, InvalidInput
        },
    }
};
pub use once_cell;
pub use near_sdk::{
    env, ext_contract, near_bindgen, AccountId,
    Balance, Promise, PromiseResult, PublicKey,
};

fn main() {
    println!("Hello, world!");
}
