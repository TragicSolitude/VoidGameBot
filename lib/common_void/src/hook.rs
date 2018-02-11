//                                      TBD------------- Filters--------- Hooks----------- Features--------
pub const NONE: u64                 = 0b0000000000000000_0000000000000000_0000000000000000_0000000000000000;
pub const VOICE_STATE_UPDATE: u64   = 0b0000000000000000_0000000000000000_0000000000000001_0000000000000000;

/// A pretty cheeky hook, this one is supposed to be called each time a plugin
/// gets loaded with a reference to said plugins for functions to be called
/// 
/// Unfortunately since there is no sense of a guaranteed load order or plugin
/// dependencies this will instead be called once all plugins have been loaded
/// and will be called for each plugin that was loaded
pub const PLUGIN_LOAD: u64          = 0b0000000000000000_0000000000000000_0000000000000010_0000000000000000;