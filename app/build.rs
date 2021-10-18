#[cfg(target_os = "windows")]
fn main() {
    if std::env::var("PROFILE").unwrap() == "release" {
        let mut res = winres::WindowsResource::new();

        res.set("ProductName", "Trilogy Save Editor");
        res.set("FileDescription", "Trilogy Save Editor");
        res.set_icon("../misc/tse.ico");
        
        if let Err(err) = res.compile() {
            eprint!("{}", err);
            std::process::exit(1);
        }
    }
}

#[cfg(not(target_os = "windows"))]
fn main() {}
