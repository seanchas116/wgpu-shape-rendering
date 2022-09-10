use cgmath::{point2, Point2};
use lyon::math::{point, Point};
use lyon::path::path::Builder;
use lyon::path::Path;
use lyon::tessellation::*;

use crate::renderer::mesh::Vertex;

pub fn example_tessellation() -> VertexBuffers<Vertex, u16> {
    // Build a Path.
    let mut builder = Path::builder();
    builder.begin(point(0.0, 0.0));
    builder.line_to(point(100.0, 0.0));
    builder.quadratic_bezier_to(point(200.0, 0.0), point(200.0, 100.0));
    builder.cubic_bezier_to(point(100.0, 100.0), point(0.0, 100.0), point(0.0, 0.0));
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

pub fn example_text(
    text: &str,
    font_size: f32,
    line_height: f32,
    x: f32,
    y: f32,
) -> VertexBuffers<Vertex, u16> {
    let font_data = include_bytes!("NotoSerifJP-Regular.otf");
    let font_face = ttf_parser::Face::from_slice(font_data, 0).unwrap();
    let rustybuzz_face = rustybuzz::Face::from_slice(font_data, 0).unwrap();

    let scale = font_size / font_face.units_per_em() as f32;

    let mut pos_x = x;
    let mut pos_y = y;

    let mut last_offset: usize = 0;

    let mut buffer = rustybuzz::UnicodeBuffer::new();
    buffer.push_str(text);
    let glyph_buffer = rustybuzz::shape(&rustybuzz_face, &[], buffer);

    let mut builder = Path::builder();

    for i in 0..glyph_buffer.len() {
        let glyph = glyph_buffer.glyph_infos()[i];
        let pos = glyph_buffer.glyph_positions()[i];

        font_face.outline_glyph(
            ttf_parser::GlyphId(glyph.glyph_id as u16),
            &mut TestOutlineBuilder {
                builder: &mut builder,
                scale,
                pos: point2(pos_x, pos_y),
            },
        );

        pos_x += pos.x_advance as f32 * scale;
    }

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

struct TestOutlineBuilder<'a> {
    builder: &'a mut Builder,
    pos: Point2<f32>,
    scale: f32,
}

impl<'a> TestOutlineBuilder<'a> {
    fn transform(&self, x: f32, y: f32) -> Point {
        point(x * self.scale + self.pos.x, y * -self.scale + self.pos.y)
    }
}

impl<'a> ttf_parser::OutlineBuilder for TestOutlineBuilder<'a> {
    fn move_to(&mut self, x: f32, y: f32) {
        //info!("move_to: {}, {}", x, y);
        self.builder.begin(self.transform(x, y));
    }

    fn line_to(&mut self, x: f32, y: f32) {
        //info!("line_to: {}, {}", x, y);
        self.builder.line_to(self.transform(x, y));
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        //info!("quad_to: {}, {}, {}, {}", x1, y1, x, y);
        self.builder
            .quadratic_bezier_to(self.transform(x1, y1), self.transform(x, y));
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        //info!("curve_to: {}, {}, {}, {}, {}, {}", x1, y1, x2, y2, x, y);
        self.builder.cubic_bezier_to(
            self.transform(x1, y1),
            self.transform(x2, y2),
            self.transform(x, y),
        );
    }

    fn close(&mut self) {
        //info!("close");
        self.builder.end(true);
    }
}
