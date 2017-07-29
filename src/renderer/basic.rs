extern crate gfx;
extern crate cgmath;
extern crate specs;

use specs::World;
use renderer::{ColorFormat, DepthFormat};
use cgmath::{SquareMatrix, Matrix4, Vector3};
use gfx::traits::FactoryExt;
use components;
use spritesheet::{Frame, Spritesheet};

gfx_defines!{
    vertex Vertex {
        pos: [f32; 2] = "a_Pos",
        uv: [f32; 2] = "a_Uv",
    }

    constant Projection {
        model: [[f32; 4]; 4] = "u_Model",
        proj: [[f32; 4]; 4] = "u_Proj",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        projection_cb: gfx::ConstantBuffer<Projection> = "b_Projection",
        tex: gfx::TextureSampler<[f32; 4]> = "t_Texture",
        out: gfx::BlendTarget<ColorFormat> = ("Target0", gfx::state::ColorMask::all(), gfx::preset::blend::ALPHA),
    }
}

#[derive(Clone)]
pub struct WindowTargets<R: gfx::Resources> {
    pub color: gfx::handle::RenderTargetView<R, ColorFormat>,
    pub depth: gfx::handle::DepthStencilView<R, DepthFormat>,
}

pub struct Basic<R: gfx::Resources> {
    pso: gfx::PipelineState<R, pipe::Meta>,
    projection: Projection,
    target: WindowTargets<R>,
}

impl<R> Basic<R>
    where R: gfx::Resources
{
    pub fn new<F>(factory: &mut F, target: &WindowTargets<R>) -> Basic<R>
        where F: gfx::Factory<R>
    {
        use gfx::traits::FactoryExt;

        let pso = factory.create_pipeline_simple(
            include_bytes!("shaders/basic.glslv"),
            include_bytes!("shaders/basic.glslf"),
            pipe::new()
        ).unwrap();

        Basic{
            pso: pso,
            projection: Projection{
                model: Matrix4::identity().into(),
                proj: get_ortho().into(),
            },
            target: (*target).clone(),
        }
    }

    pub fn reset_transform(&mut self) {
        self.projection.model = Matrix4::identity().into();
    }

    pub fn render<C, F>(&mut self,
        encoder: &mut gfx::Encoder<R, C>,
        world: &World,
        factory: &mut F,
        transform: &components::Transform,
        frame_name: &String,
        spritesheet: &Spritesheet,
        texture: &gfx::handle::ShaderResourceView<R, [f32; 4]>)
        where R: gfx::Resources, C: gfx::CommandBuffer<R>, F: gfx::Factory<R>
    {
        use std::ops::Deref;

        let camera_res = world.read_resource::<components::Camera>();
        let camera = camera_res.deref();
        let x = transform.pos.x as f32;
        let y = transform.pos.y as f32;
        let w = transform.size.x as f32;
        let h = transform.size.y as f32;

        let region = spritesheet.frames.iter().filter(|frame|
            frame.filename == *frame_name
        ).collect::<Vec<&Frame>>()[0];
        let sw = spritesheet.meta.size.w as f32;
        let sh = spritesheet.meta.size.h as f32;
        let tx = region.frame.x as f32 / sw;
        let ty = region.frame.y as f32 / sh;
        let tx2 = (region.frame.x as f32 + region.frame.w as f32) / sw;
        let ty2 = (region.frame.y as f32 + region.frame.h as f32) / sh;

        let data: Vec<Vertex> = vec![
            Vertex{
                pos: [x, y],
                uv: [tx, ty2],
            },
            Vertex{
                pos: [x + w, y],
                uv: [tx2, ty2],
            },
            Vertex{
                pos: [x + w, y + h],
                uv: [tx2, ty],
            },
            Vertex{
                pos: [x, y + h],
                uv: [tx, ty],
            }
        ];

        let index_data: Vec<u32> = vec![0, 1, 2, 2, 3, 0];
        let (vbuf, slice) = factory.create_vertex_buffer_with_slice(&data, &index_data[..]);

        let params = pipe::Data{
            vbuf: vbuf,
            projection_cb: factory.create_constant_buffer(1),
            tex: (texture.clone(), factory.create_sampler_linear()),
            out: self.target.color.clone(),
        };

        self.projection.proj = (*camera).0.into();

        encoder.update_constant_buffer(&params.projection_cb, &self.projection);
        encoder.draw(&slice, &self.pso, &params);
    }
}

pub fn get_ortho() -> Matrix4<f32> {
    let dim = get_dimensions();
    cgmath::ortho(
        0.0, dim[0],
        0.0, dim[1],
        0.0, 1.0,
    )
}

pub fn get_dimensions() -> [f32; 2] {
    [960.0, 640.0]
}