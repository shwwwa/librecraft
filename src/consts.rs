/** Librecraft's main hard-coded constants. */
pub mod consts {
    /** Fixed time clock - leave it at 50 hz. */
    pub const FIXED_TIME_CLOCK: f64 = 50.;
    /** Minecraft protocol version that we are trying to support. */
    pub const PROTOCOL_VERSION: u32 = 758;
    /** Title of the main program. */
    pub const TITLE: &str = env!("CARGO_PKG_NAME");
    /** Version of the main program. */
    pub const VERSION: &str = env!("CARGO_PKG_VERSION");
    /** Librecraft's minimum resolution width. */
    pub const MIN_WIDTH: f32 = 320.;
    /** Librecraft's minimum resolution height. */
    pub const MIN_HEIGHT: f32 = 240.;
}
