use libloading::Library;

/// 
pub struct Plugin {
    pub link: Library,
    pub description: u64,
    pub name: String
}

impl Plugin {
    pub fn new(name: String, lib: Library) -> Plugin {
        let plugin_desc: u64;

        {
            let option = unsafe {
                lib.get::<extern fn() -> u64>(b"describe")
            };

            plugin_desc = match option {
                Ok(function) => function(),
                Err(_) => 0
            };
        }

        Plugin {
            link: lib,
            description: plugin_desc,
            name: name
        }
    }
}