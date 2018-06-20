use cgmath::Vector2;
use lyon_path::{builder::FlatPathBuilder, default::Path, math::point as lyon_point};
use lyon_tessellation::{geometry_builder::{BuffersBuilder, VertexBuffers, VertexConstructor},
                        FillOptions,
                        FillTessellator,
                        FillVertex};
use renderer::Vertex;
use specs::{Component, VecStorage};

pub struct Shape {
    pub buffers: VertexBuffers<Vertex>,
}

impl Shape {
    pub fn new(points: Vec<Vector2<f32>>, color: [f32; 4]) -> Self {
        Shape {
            buffers: Self::build_buffers(points, color),
        }
    }

    pub fn build_buffers(points: Vec<Vector2<f32>>, color: [f32; 4]) -> VertexBuffers<Vertex> {
        let mut path_builder = Path::builder();
        for (i, point) in points.iter().enumerate() {
            let p = lyon_point(point.x, point.y);
            if i == 0 {
                path_builder.move_to(p);
            } else {
                path_builder.line_to(p);
            }
        }

        path_builder.close();

        let path = path_builder.build();
        let mut buffers = VertexBuffers::new();

        // Create the tessellator.
        let mut tessellator = FillTessellator::new();

        // Compute the tessellation.
        tessellator
            .tessellate_path(
                path.path_iter(),
                &FillOptions::default(),
                &mut BuffersBuilder::new(&mut buffers, VertexCtor { color }),
            )
            .unwrap();

        buffers
    }
}

struct VertexCtor {
    pub color: [f32; 4],
}

impl VertexConstructor<FillVertex, Vertex> for VertexCtor {
    fn new_vertex(&mut self, vertex: FillVertex) -> Vertex {
        Vertex {
            pos: [vertex.position.x, vertex.position.y, 0.0],
            uv: [0.0, 0.0],
            color: self.color.clone(),
        }
    }
}

impl Component for Shape {
    type Storage = VecStorage<Self>;
}
