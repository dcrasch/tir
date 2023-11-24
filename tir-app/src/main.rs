#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;
use tir_app::components::tessellationeditor::TessellationEditor;

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    render!(
        rect {

            TessellationEditor {
                offset_x: 200.0,
                offset_y: 200.0,
                scale: 200.0,
                //background: "black",
                width: "100%",
                height: "100%"
            }
        }
    )
}
