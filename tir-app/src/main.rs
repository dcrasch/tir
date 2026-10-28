#[cfg(not(target_os = "android"))]
fn main() -> eframe::Result {
    tir_app::run_desktop()
}

#[cfg(target_os = "android")]
fn main() {}
