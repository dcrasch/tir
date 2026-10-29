//! Desktop entry point.

use crate::app::FontScale;

/// Desktop entry point (used by src/main.rs).
pub fn run_desktop() -> eframe::Result {
    env_logger::init();
    let options = eframe::NativeOptions {
        renderer: eframe::Renderer::Wgpu,
        viewport: eframe::egui::ViewportBuilder::default()
            .with_title("Tessellations")
            .with_inner_size([800.0, 700.0]),
        ..Default::default()
    };
    // No system setting on desktop; `TIR_FONT_SCALE=1.3` simulates Android's.
    let font_scale = std::env::var("TIR_FONT_SCALE")
        .ok()
        .and_then(|v| v.parse::<f32>().ok())
        .unwrap_or(1.0);
    eframe::run_native(
        "Tessellations",
        options,
        Box::new(move |cc| create_app(cc, font_scale)),
    )
}

type AppResult = Result<Box<dyn eframe::App>, Box<dyn std::error::Error + Send + Sync>>;

fn create_app(cc: &eframe::CreationContext<'_>, font_scale: f32) -> AppResult {
    // Android: egui already scales by the device density (pixels_per_point),
    // so no extra zoom factor is needed.
    let _ = cc;
    let font_scale = FontScale::from_raw(font_scale);
    log::info!(
        "font scale: {} (x{:.2})",
        font_scale.label(),
        font_scale.factor()
    );
    #[allow(unused_mut)]
    let mut app = crate::app::TirApp {
        font_scale,
        ..crate::app::TirApp::default()
    };
    #[cfg(feature = "screenshot")]
    if let Some(i) = std::env::var("TIR_FIGURE")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
    {
        app.kind = crate::figure::FigureKind::ALL[i % crate::figure::FigureKind::ALL.len()];
        app.reset();
    }
    #[cfg(feature = "screenshot")]
    if std::env::var_os("TIR_DEMO").is_some() {
        app.demo_edit();
    }
    #[cfg(feature = "screenshot")]
    if std::env::var_os("TIR_PICKER").is_some() {
        app.apply(crate::ui::MenuAction::PickSymmetry);
    }
    Ok(Box::new(app))
}
