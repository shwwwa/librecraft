#[cfg(debug_assertions)]
pub const DEBUG_MODE: bool = true;
#[cfg(debug_assertions)]
pub const LOG_FILTER: &str = "warn,wgpu_core=warn,wgpu_hal=off,bevy_diagnostic=off,client=debug";
#[cfg(not(debug_assertions))]
pub const DEBUG_MODE: bool = false;
#[cfg(not(debug_assertions))]
pub const LOG_FILTER: &str = "warn,wgpu_core=warn,wgpu_hal=off,bevy_diagnostic=warn,client=info";

/** Path to asset folder. */
pub const ASSET_FOLDER: &str = "../assets";
/** Fixed time clock - leave it at 50 hz. */
pub const FIXED_TIME_CLOCK: f64 = 50.;
/** Minecraft protocol version that we are trying to support. */
pub const PROTOCOL_VERSION: u32 = 758;
/** Path to the settings file when on debug. */
pub const DEBUG_SETTINGS_PATH: &str = "./";
/** Title of the main program. */
macro_rules! title {
    () => {
        env!("CARGO_PKG_NAME")
    };
}
/** Version of the main program. */
macro_rules! version {
    () => {
        env!("CARGO_PKG_VERSION")
    };
}
/** Version string. */
macro_rules! version_string {
    () => {
        concat!(crate::consts::title!(), " v.", crate::consts::version!())
    };
}
/** About (developer and version string) */
#[allow(unused_macros)]
macro_rules! about {
    () => {
        concat!(crate::consts::version_string!(), " | by caffidev")
    };
}
pub(crate) use about;
pub(crate) use title;
pub(crate) use version;
pub(crate) use version_string;

/** Librecraft's minimum resolution width. */
pub const MIN_WIDTH: f32 = 320.;
/** Librecraft's minimum resolution height. */
pub const MIN_HEIGHT: f32 = 240.;
