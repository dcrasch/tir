//! Dev helper: `TIR_SCREENSHOT=out.png` saves the 3rd frame and exits.

use eframe::egui;
use std::sync::atomic::{AtomicU32, Ordering};

static FRAME: AtomicU32 = AtomicU32::new(0);

pub fn handle(ctx: &egui::Context) {
    let Some(path) = std::env::var_os("TIR_SCREENSHOT") else {
        return;
    };
    let n = FRAME.fetch_add(1, Ordering::Relaxed);
    if n == 3 {
        ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(Default::default()));
    }
    ctx.request_repaint();
    let image = ctx.input(|i| {
        i.events.iter().find_map(|e| match e {
            egui::Event::Screenshot { image, .. } => Some(image.clone()),
            _ => None,
        })
    });
    if let Some(img) = image {
        let file = std::fs::File::create(&path).expect("create screenshot");
        let mut enc = png::Encoder::new(file, img.size[0] as u32, img.size[1] as u32);
        enc.set_color(png::ColorType::Rgba);
        enc.set_depth(png::BitDepth::Eight);
        enc.write_header()
            .unwrap()
            .write_image_data(img.as_raw())
            .unwrap();
        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
    }
}
