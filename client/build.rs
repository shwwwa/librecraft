#[cfg(target_os = "windows")]
extern crate winresource;

fn main() {
    // Double-check to exclude errors on compile.
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        #[cfg(target_os = "windows")]
        {
            let mut res = winresource::WindowsResource::new();
            res.set_icon("../assets/icon/icon64.ico");
            res.compile().unwrap();
        }
    }
}
