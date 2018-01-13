//                                      TBD------------- Filters--------- Hooks----------- Features--------
pub const NONE: u64                 = 0b0000000000000000_0000000000000000_0000000000000000_0000000000000000;

/// This filter gets called after the command is parsed from the message and
/// just before the plugin gets looked up and ran/loaded
///
/// The function will be sent 1 argument
/// - command: &mut str | The command without the leading COMMAND_PREFIX
pub const BEFORE_PLUGIN_LOOKUP: u64 = 0b0000000000000000_0000000000000001_0000000000000000_0000000000000000;