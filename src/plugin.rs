extern crate libloading;

use libloading::Library;

/// 
pub struct Plugin {
    pub link: Library,
    pub description: u32
}

impl Plugin {
    pub fn new(lib: Library) -> Plugin {
        let plugin_desc: u32;

        {
            let option = unsafe {
                lib.get::<extern fn() -> u32>(b"describe")
            };

            plugin_desc = match option {
                Ok(function) => function(),
                Err(_) => 0
            };
        }

        Plugin {
            link: lib,
            description: plugin_desc
        }
    }
}