use cgmath::point2;
use lyon::math::point;
use lyon::path::Path;
use lyon::tessellation::*;

use crate::renderer::mesh::Vertex;

pub fn exampleTessellation() -> VertexBuffers<Vertex, u16> {
    // Build a Path.
    let mut builder = Path::builder();
    builder.begin(point(0.0, 0.0));
    builder.line_to(point(10.0, 0.0));
    builder.quadratic_bezier_to(point(20.0, 0.0), point(20.0, 10.0));
    builder.cubic_bezier_to(point(10.0, 10.0), point(0.0, 10.0), point(0.0, 0.0));
    builder.end(true);
    let path = builder.build();

    // Will contain the result of the tessellation.
    let mut geometry: VertexBuffers<Vertex, u16> = VertexBuffers::new();
    let mut tessellator = FillTessellator::new();
    {
        // Compute the tessellation.
        tessellator
            .tessellate_path(
                &path,
                &FillOptions::default(),
                &mut BuffersBuilder::new(&mut geometry, |vertex: FillVertex| Vertex {
                    position: point2(vertex.position().x, vertex.position().y),
                }),
            )
            .unwrap();
    }

    return geometry;
}
