# VOIDGAMEBOT

This bot uses a plugin system to create a BASH like
`command arg[0] arg[1] arg[n]` setup that can be extended simply by adding
another plugin at runtime to add a new command.

## USAGE

The command prefix is currently hardcoded to `!` (e.g. `!command args[..]`) and
commands can be run in any text channel the bot has permission to view. With
that said it is generally preferrable to keep commands in DMs to the bot if they
don't act on any particular text channel so that spam can be kept to a minimum.

**The commands built in to the bot are:**

#

### !PING

Was initially just used as a test when developing the command system
but is left in there because why not?

#

### !PLAYING (GAME)

When called while connected to a voice channel will create a new voice channel
in the same server with the name specified `(GAME)` and will move you into the
new channel. This channel will exist only while there is at least one user in
it. Once the last person has left the channel it will be deleted.

There are some conditions that need to be met for the command to execute
successfully:

1. You must be connected to a voice channel that is visible to the bot; the bot
cannot force your discord client to connect.
2. The voice channel can't have spaces because spaces are used to separate
arguments. It won't necessarily break the command but it will just use the first
word as the new channel name

#### ARGUMENTS

- (GAME) : The game you are playing; this will be the name of the new voice
channel that gets created

#### ERROR CODES

- `100` Invalid argument count
- `200` Error locking channels map
- `201` Error locking prev_channels map
- `300` User state unknown
- `400` Could not create channel
- `401` Could not get channel info
- `402` User channel not accessible
- `403` Wrong channel type created
- `404` Channel not found -- wow how funny am I a 404 joke /s
- `500` Could not move user into channel

#### KNOWN BUGS

These are all of the bugs currently known about. This doesn't mean that the
following are *all* of the bugs that exist, just the ones that are known. If a
bug is found feel free to create an issue in BitBucket or DM _OffensiveBias

1. The plugin does not sync the inital voice state of any users that may already
be connected to a voice channel when it starts up. If the bot starts up while
someone is already connected to a visible voice channel it will not recognize
their existence in said channel until they disconnect/reconnect or move to
another channel first.

#

## DEVELOPMENT

The bot is open source under the MIT license. (tl;dr You can pretty much do
whatever as long as you include the original copyright and license in any copy
of this source/software). The repository can be found on
[bitbucket](https://bitbucket.org/noahshuart/voidgamebot/overview). More details
about plugin development can be found in `README.md`.

## KNOWN BUGS

These are all of the bugs currently known about. This doesn't mean that the
following are *all* of the bugs that exist, just the ones that are known. If a
bug is found feel free to create an issue in BitBucket or DM _OffensiveBias

1. The bot will crash if it is added to a new server while it is running.