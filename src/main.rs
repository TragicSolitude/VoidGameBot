extern crate libloading;
extern crate regex;

mod platform;

use libloading::{Library, Symbol};
use std::fs;
use std::collections::HashMap;
use platform::Platform;

fn main() {
    let plugins = fs::read_dir("plugins/").unwrap();
    let mut libs: HashMap<String, Library> = HashMap::new();

    // TODO Platform detection for lib file extensions
    let platform = Platform::NIX;

    for plugin in plugins {
        let plugin = plugin.unwrap();
        let mut key = plugin.file_name().into_string().unwrap();

        // Cut off file extension
        let len = key.len();
        key.truncate(len - 3);

        let lib = Library::new(plugin.path()).unwrap();
        libs.insert(key, lib);
    }

    for pair in libs.into_iter() {
        let (key, lib) = pair;
        // We're gonna need a shitton of checks and error handling with this lib
        // stuff
        unsafe {
            let func: Symbol<unsafe extern fn() -> i16> = lib.get(b"main").unwrap();

            println!("Result of {}.main() -- {}", key, func());
        }
    }
}