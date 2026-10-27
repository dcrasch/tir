mod coloring;
mod figure;
mod plane;
mod registry;
mod spec;
mod tiles;

pub use coloring::{color_table, ColorTable};
pub use figure::{EdgeLink, Placement, TessellationFigure};
pub use plane::TessellationPlane;
pub use registry::{FigureInfo, FIGURES, FIGURE_IDS};
pub use spec::{from_polygon, from_polygon_with_mirrors, EdgeKind, MirrorAxis, Pairing};
