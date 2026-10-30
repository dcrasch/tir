//! Escher-style tessellation editor built on the `tessellations` crate
//! (https://github.com/dcrasch/tir), rendered with egui + wgpu.
//!
//! - Background: the figure tiled over the plane (filled, 4-color palette).
//! - Foreground: the editable figure outline in green, with handles.
//! - Press on a handle and drag to move it. Press on an edge to insert a new
//!   point there and drag it. The corresponding (opposite) edge follows
//!   automatically because that's how `TessellationFigure` stores lines.

// Modules
mod app;
mod figure;
mod input;
mod palette_picker;
mod picker;
mod render;
mod ui;

pub mod platform;

#[cfg(feature = "screenshot")]
mod utilities;

// Re-exports
pub use app::{FontScale, TirApp};
pub use figure::{FigureKind, Palette, View};

#[cfg(not(target_os = "android"))]
pub use platform::run_desktop;
