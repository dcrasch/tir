use tessellations::tessellationfigure::TessellationFigure;

use fermi::*;

pub static DATA: AtomRef<TessellationFigure> = AtomRef(|_| TessellationFigure::triangle());
