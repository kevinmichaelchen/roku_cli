use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct RokuArgs {
    #[command(subcommand)]
    pub action_type: ActionType,
}

#[derive(Debug, Subcommand)]
pub enum ActionType {
    /// Discovers Roku instances on your local area network.
    Discover(DiscoverCommand),

    /// Simulates remote key presses (e.g., up, down, left, right, etc.)
    // Full list here: https://developer.roku.com/docs/developer-program/debugging/external-control-api.md#keypress-key-values
    Key(KeyCommand),
}

#[derive(Debug, Args)]
pub struct DiscoverCommand {/* Empty for now, but could */}

#[derive(Debug, Args)]
pub struct KeyCommand {
    #[command(subcommand)]
    pub key_type: KeyType,
}

#[derive(Debug, Subcommand)]
pub enum KeyType {
    /// Go to the Home screen
    Home,

    /// Rewind
    Rev,

    /// Fast-forward
    Fwd,

    /// Play
    Play,

    /// Select
    Select,

    /// Left
    Left,

    /// Right
    Right,

    /// Down
    Down,

    /// Up
    Up,

    /// Back
    Back,

    /// Instant replay
    InstantReplay,

    /// Info
    Info,

    /// Backspace
    Backspace,

    /// Search
    Search,

    /// Enter
    Enter,

    /// Emit the remote's high-pitched beep to locate it when it's lost.
    // Not all remotes support this feature. When pressed, this button will
    // automatically turn on your TV and start emitting a high-pitched beeping
    // sound so you can locate your lost remote.
    // The query/device-info endpoint includes a supports-find-remote flag that
    // indicates whether the Roku device supports FindRemote.
    FindRemote,

    /// Turns the volume down.
    /// Only supported on some devices (e.g., Roku TVs).
    VolumeDown,

    /// Mutes the volume.
    /// Only supported on some devices (e.g., Roku TVs).
    VolumeMute,

    /// Turns the volume up.
    /// Only supported on some devices (e.g., Roku TVs).
    VolumeUp,

    /// Powers off the device.
    /// Only supported on some devices (e.g., Roku TVs).
    PowerOff,

    /// Changes the channel up (when watching the TV "Tuner" input).
    /// (Only supported on Roku TVs).
    ChannelUp,

    /// Changes the channel down (when watching the TV "Tuner" input).
    /// (Only supported on Roku TVs).
    ChannelDown,

    /// Changes the input to the TV "Tuner".
    /// (Only supported on Roku TVs).
    InputTuner,

    /// Changes the input to HDMI 1.
    /// (Only supported on Roku TVs).
    InputHDMI1,

    /// Changes the input to HDMI 2.
    /// (Only supported on Roku TVs).
    InputHDMI2,

    /// Changes the input to HDMI 3.
    /// (Only supported on Roku TVs).
    InputHDMI3,

    /// Changes the input to HDMI 4.
    /// (Only supported on Roku TVs).
    InputHDMI4,

    /// Changes the input to AV 1.
    /// (Only supported on Roku TVs).
    InputAV1,
}
