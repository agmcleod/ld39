extern crate cgmath;
extern crate gfx;
extern crate specs;

use std::ops::Deref;

use cgmath::{Matrix4, SquareMatrix, Transform};
use components;
use gfx::texture;
use gfx::traits::FactoryExt;
use gfx_glyph::{GlyphBrush, Section};
use renderer::{ColorFormat, DepthFormat};
use specs::World;
use spritesheet::{Frame, Spritesheet};

gfx_defines!{
    vertex Vertex {
        pos: [f32; 3] = "a_Pos",
        uv: [f32; 2] = "a_Uv",
        color: [f32; 4] = "a_Color",
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
        depth: gfx::DepthTarget<DepthFormat> = gfx::preset::depth::LESS_EQUAL_WRITE,
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
    model: Matrix4<f32>,
    pub target: WindowTargets<R>,
    color_texture: (
        gfx::handle::ShaderResourceView<R, [f32; 4]>,
        gfx::handle::Sampler<R>,
    ),
}

impl<R> Basic<R>
where
    R: gfx::Resources,
{
    pub fn new<F>(factory: &mut F, target: WindowTargets<R>) -> Basic<R>
    where
        F: gfx::Factory<R>,
    {
        use gfx::traits::FactoryExt;

        let pso = factory
            .create_pipeline_simple(
                include_bytes!("shaders/basic.glslv"),
                include_bytes!("shaders/basic.glslf"),
                pipe::new(),
            )
            .unwrap();

        let texels = [[0xff, 0xff, 0xff, 0xff]];
        let (_, texture_view) = factory
            .create_texture_immutable::<ColorFormat>(
                texture::Kind::D2(1, 1, texture::AaMode::Single),
                texture::Mipmap::Allocated,
                &[&texels],
            )
            .unwrap();

        let sinfo =
            texture::SamplerInfo::new(texture::FilterMethod::Mipmap, texture::WrapMode::Clamp);

        let dim = self::super::get_dimensions();

        Basic {
            pso: pso,
            projection: Projection {
                model: Matrix4::identity().into(),
                proj: self::super::get_ortho(dim[0] as f32, dim[1] as f32).into(),
            },
            model: Matrix4::identity(),
            target,
            color_texture: (texture_view, factory.create_sampler(sinfo)),
        }
    }

    fn draw_verticies<F, C>(
        &mut self,
        data: Vec<Vertex>,
        encoder: &mut gfx::Encoder<R, C>,
        factory: &mut F,
        tex: (
            gfx::handle::ShaderResourceView<R, [f32; 4]>,
            gfx::handle::Sampler<R>,
        ),
        camera: &components::Camera,
    ) where
        R: gfx::Resources,
        C: gfx::CommandBuffer<R>,
        F: gfx::Factory<R>,
    {
        let index_data: Vec<u32> = vec![0, 1, 2, 2, 3, 0];
        let (vbuf, slice) = factory.create_vertex_buffer_with_slice(&data, &index_data[..]);

        let params = pipe::Data {
            vbuf: vbuf,
            projection_cb: factory.create_constant_buffer(1),
            tex: tex,
            out: self.target.color.clone(),
            depth: self.target.depth.clone(),
        };

        self.projection.proj = (*camera).0.into();

        self.projection.model = self.model.into();

        encoder.update_constant_buffer(&params.projection_cb, &self.projection);
        encoder.draw(&slice, &self.pso, &params);
    }

    pub fn reset_transform(&mut self) {
        self.model = Matrix4::identity().into();
    }

    pub fn render_single_texture<C, F>(
        &mut self,
        encoder: &mut gfx::Encoder<R, C>,
        world: &World,
        factory: &mut F,
        transform: &components::Transform,
        texture: &gfx::handle::ShaderResourceView<R, [f32; 4]>,
        color: &components::Color,
    ) where
        R: gfx::Resources,
        C: gfx::CommandBuffer<R>,
        F: gfx::Factory<R>,
    {
        let camera_res = world.read_resource::<components::Camera>();
        let camera = camera_res.deref();
        let w = transform.size.x as f32;
        let h = transform.size.y as f32;

        let tx = 0.0;
        let ty = 0.0;
        let tx2 = 1.0;
        let ty2 = 1.0;

        let tex = (
            texture.clone(),
            factory.create_sampler(texture::SamplerInfo::new(
                texture::FilterMethod::Scale,
                texture::WrapMode::Clamp,
            )),
        );

        let data: Vec<Vertex> = get_quad(color.0, w, h, tx, ty, tx2, ty2);

        self.draw_verticies(data, encoder, factory, tex, &camera);
    }

    pub fn render<C, F>(
        &mut self,
        encoder: &mut gfx::Encoder<R, C>,
        world: &World,
        factory: &mut F,
        transform: &components::Transform,
        frame_name: Option<&String>,
        spritesheet: &Spritesheet,
        color: Option<&components::Color>,
        texture: Option<&gfx::handle::ShaderResourceView<R, [f32; 4]>>,
    ) where
        R: gfx::Resources,
        C: gfx::CommandBuffer<R>,
        F: gfx::Factory<R>,
    {
        let camera_res = world.read_resource::<components::Camera>();
        let camera = camera_res.deref();
        let w = transform.size.x as f32;
        let h = transform.size.y as f32;

        let mut tx = 0.0;
        let mut ty = 0.0;
        let mut tx2 = 1.0;
        let mut ty2 = 1.0;

        if let Some(frame_name) = frame_name {
            let region = spritesheet
                .frames
                .iter()
                .filter(|frame| frame.filename == *frame_name)
                .collect::<Vec<&Frame>>()[0];
            let sw = spritesheet.meta.size.w as f32;
            let sh = spritesheet.meta.size.h as f32;
            tx = region.frame.x as f32 / sw;
            ty = region.frame.y as f32 / sh;
            tx2 = (region.frame.x as f32 + region.frame.w as f32) / sw;
            ty2 = (region.frame.y as f32 + region.frame.h as f32) / sh;
        }

        let tex: (
            gfx::handle::ShaderResourceView<R, [f32; 4]>,
            gfx::handle::Sampler<R>,
        ) = if let Some(texture) = texture {
            (
                texture.clone(),
                factory.create_sampler(texture::SamplerInfo::new(
                    texture::FilterMethod::Bilinear,
                    texture::WrapMode::Clamp,
                )),
            )
        } else {
            self.color_texture.clone()
        };

        let color = if let Some(color) = color {
            color.0
        } else {
            [1.0; 4]
        };

        let data: Vec<Vertex> = get_quad(color, w, h, tx, ty, tx2, ty2);

        self.draw_verticies(data, encoder, factory, tex, &camera);
    }

    pub fn render_shape<C, F>(
        &mut self,
        encoder: &mut gfx::Encoder<R, C>,
        world: &World,
        factory: &mut F,
        shape: &components::Shape,
    ) where
        R: gfx::Resources,
        C: gfx::CommandBuffer<R>,
        F: gfx::Factory<R>,
    {
        use std::ops::Deref;

        let camera_res = world.read_resource::<components::Camera>();
        let camera = camera_res.deref();

        let buffers = &shape.buffers;
        let (vbuf, slice) =
            factory.create_vertex_buffer_with_slice(&buffers.vertices[..], &buffers.indices[..]);

        let params = pipe::Data {
            vbuf: vbuf,
            projection_cb: factory.create_constant_buffer(1),
            tex: self.color_texture.clone(),
            out: self.target.color.clone(),
            depth: self.target.depth.clone(),
        };

        self.projection.proj = (*camera).0.into();
        self.projection.model = self.model.into();

        encoder.update_constant_buffer(&params.projection_cb, &self.projection);
        encoder.draw(&slice, &self.pso, &params);
    }

    pub fn render_text<C, F>(
        &mut self,
        encoder: &mut gfx::Encoder<R, C>,
        text: &components::Text,
        transform: &components::Transform,
        color: &components::Color,
        glyph_brush: &mut GlyphBrush<R, F>,
        hidpi_factor: f32,
        scale_from_base_res: &(f32, f32),
    ) where
        R: gfx::Resources,
        C: gfx::CommandBuffer<R>,
        F: gfx::Factory<R>,
    {
        let absolute_pos = transform.get_absolute_pos();
        let mut scale = text.scale.clone();
        scale.x *= hidpi_factor * scale_from_base_res.0;
        scale.y *= hidpi_factor * scale_from_base_res.1;
        let section = Section {
            text: text.text.as_ref(),
            scale,
            bounds: (
                text.size.x as f32 * hidpi_factor,
                text.size.y as f32 * hidpi_factor,
            ),
            screen_position: (
                absolute_pos.x * hidpi_factor * scale_from_base_res.0,
                absolute_pos.y * hidpi_factor * scale_from_base_res.1,
            ),
            color: color.0,
            z: 0.0,
            ..Section::default()
        };

        glyph_brush.queue(section);
        glyph_brush
            .draw_queued(encoder, &self.target.color, &self.target.depth)
            .unwrap();
    }

    pub fn transform(&mut self, transform: &components::Transform, undo: bool) {
        let mut transform_mat = Matrix4::from_translation(*transform.get_pos());
        let mut scale = Matrix4::from_nonuniform_scale(
            transform.scale.x,
            transform.scale.y,
            1.0,
        );
        if undo {
            transform_mat = transform_mat.inverse_transform().unwrap();
            scale = scale.inverse_transform().unwrap();
            self.model = self.model.concat(&scale).concat(&transform_mat);
        } else {
            self.model = self.model.concat(&transform_mat).concat(&scale);
        }

    }
}

fn get_quad(color: [f32; 4], w: f32, h: f32, tx: f32, ty: f32, tx2: f32, ty2: f32) -> Vec<Vertex> {
    vec![
        Vertex {
            pos: [0.0, 0.0, 0.0],
            uv: [tx, ty],
            color: color,
        },
        Vertex {
            pos: [w, 0.0, 0.0],
            uv: [tx2, ty],
            color: color,
        },
        Vertex {
            pos: [w, h, 0.0],
            uv: [tx2, ty2],
            color: color,
        },
        Vertex {
            pos: [0.0, h, 0.0],
            uv: [tx, ty2],
            color: color,
        },
    ]
}
