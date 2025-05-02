use vergen_git2::{BuildBuilder, Emitter, Git2Builder};

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

    #[cfg(feature = "vergen")]
    {
        let build = BuildBuilder::default().build_date(true).build().unwrap();
        let git2 = Git2Builder::default().sha(true).build().unwrap();

        Emitter::default()
            .add_instructions(&build)
            .unwrap()
            .add_instructions(&git2)
            .unwrap()
            .emit()
            .unwrap();
    }
}
