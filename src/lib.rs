// use std::char::ToLowercase;
// Nscript v2 ( remade from au3 nscript) by Nick Hagen.
//use std::collections::HashMap;
//use std::{env, array, string};
use std::fs;
use std::fs::File;
use std::path::Path;
use std::io::{self,Read, Write};
use std::net::TcpStream;
use std::process;
use std::process::Command;
use std::time::Duration;
// use url::Url;
use colored::Colorize;
use std::net::ToSocketAddrs;
use rand::seq::SliceRandom;
use encoding_rs::UTF_8;
use std::env;

//time
use chrono::{Datelike, Timelike};

use std::net::TcpListener;

use hex::FromHex;
//use regex::Regex;
use std::thread;
// use reqwest;
// use hex::FromHex;
//use regex::Regex;
// use std::thread;
//mod nscriptapilib;

// use includes::nscript_time::*;
// use includes::nscript_http_html::*;
// use includes::nscript_rust_fn_bindings::*;
// use includes::nscript_zip::*;
// use includes::nscript_interpreter::*;
// use includes::nscript_api_lib::*;
// use includes::nscript_functions::*;
// use includes::nscript_strings::*;
// use includes::nscript_arrays::*;
// use includes::nscript_file_and_system::*;
use rand::Rng;
#[cfg(windows)]
mod ioctlsocket {
    use std::os::windows::raw::SOCKET;
    use std::os::raw::{c_long, c_ulong};

    extern "system" {
        pub fn ioctlsocket(s: SOCKET, cmd: c_long, argp: *mut c_ulong) -> i32;
    }
}

//#[cfg(not(windows))]
//use std::os::unix::io::AsRawFd;
pub const NSCRIPT_VERSION: &'static str = "1.036";
// const NSCRIPT_INFO: &'static str = "
// Nscript core in Rust-language.
// Created by Nick Hagen.
// 2022-23";
#[cfg(windows)]
const NC_LINE_ENDING: &'static str = "\n";
#[cfg(not(windows))]
pub const NC_LINE_ENDING: &'static str = "\n";
pub const CODE_NC_LINE_ENDING: &'static str = "\n";
#[cfg(windows)]
const MACRO_OS: &'static str = "Windows";
#[cfg(not(windows))]
pub const MACRO_OS: &'static str = "Unix";
pub const NC_SERVER_ADDRESS: &str = "0.0.0.0";
pub const NC_SERVER_PORT: u16 = 8088;
#[cfg(not(windows))]
pub const NC_SERVER_ROOT: &str = "./public/";
#[cfg(windows)]
pub const NC_SERVER_ROOT: &str = ".\\public\\";
#[cfg(not(windows))]
pub const NC_SCRIPT_DIR : &str = "./";
#[cfg(windows)]
pub const NC_SCRIPT_DIR: &str = ".\\";

//use std::path::{PathBuf, Path};

mod includes {
    pub mod nscript_zip;
    //pub mod nscript_api_lib;
    pub mod nscript_functions;
    pub mod nscript_arrays;
    pub mod nscript_file_and_system;
    pub mod nscript_strings;
    pub mod nscript_interpreter;
    pub mod nscript_rust_fn_bindings;
    pub mod nscript_http_html;
    pub mod nscript_time;
    pub mod nscript_networking;
    pub mod nscript_sound;
    //pub mod nscript_egui;
}

pub use includes::nscript_zip::*;
//pub use includes::nscript_api_lib::*;
pub use includes::nscript_functions::*;
pub use includes::nscript_arrays::*;
pub use includes::nscript_file_and_system::*;
pub use includes::nscript_strings::*;
pub use includes::nscript_interpreter::*;
pub use includes::nscript_rust_fn_bindings::*;
pub use includes::nscript_http_html::*;
pub use includes::nscript_time::*;

pub use includes::nscript_networking::*;
pub use includes::nscript_sound::*;
//pub use core::fmt::Error;
//pub use includes::nscript_egui::*;








// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
