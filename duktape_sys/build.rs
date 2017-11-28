#![feature(os)]
#![feature(path)]
#![feature(collections)]

extern crate gcc;

use std::default::Default;
use std::path::Path;
use std::env;

fn main() {
    // Make sure we get a thread-safe build.  Without this, duktape refuses
    // to set DUK_USE_VARIADIC_MACROS and falls back to global variables.
    let mut cflags = env::var("CFLAGS").unwrap_or("".to_string());
    cflags.push_str(" -std=c99");
    env::set_var("CFLAGS", cflags);

    let mut build = gcc::Build::new();

    build
        .file("duktape/src/duktape.c")
        .file("src/glue.c")
        .include("duktape/src")
        .compile("libduktape.a");
}
