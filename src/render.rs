use posh::{gl, sl, Block, BlockDom, Gl, Sl, VsInterface, VsInterfaceDom};

use crate::{
    arrow_positions,
    shader::{fragment_shader, vertex_shader},
    SCREEN_SIZE,
};

#[derive(Clone, Copy, Block)]
#[repr(C)]
pub struct Camera<D: BlockDom> {
    pub world_to_view: D::Mat4,
    pub view_to_screen: D::Mat4,
}

#[derive(Clone, Copy, Block)]
#[repr(C)]
pub struct Instance<D: BlockDom> {
    pub model_to_view: D::Mat4,
    pub color: D::Vec3,
}

#[derive(Copy, Clone, VsInterface)]
pub struct VsInput<D: VsInterfaceDom> {
    pub instance: D::Block<Instance<Sl>>,
    pub model_pos: D::Block<sl::Vec3>,
}

pub struct Graphics {
    program: gl::Program<Camera<Sl>, VsInput<Sl>>,

    camera: gl::UniformBuffer<Camera<Gl>>,

    pub instances: gl::VertexBuffer<Instance<Gl>>,
    arrow: gl::VertexBuffer<gl::Vec3>,
}

impl Graphics {
    pub fn new(gl: gl::Context) -> Result<Self, gl::CreateError> {
        use gl::BufferUsage::*;

        Ok(Self {
            program: gl.create_program(vertex_shader, fragment_shader)?,
            camera: gl.create_uniform_buffer(Camera::default(), StaticDraw)?,
            instances: gl.create_vertex_buffer(&[], StreamDraw)?,
            arrow: gl.create_vertex_buffer(&arrow_positions(), StaticDraw)?,
        })
    }

    pub fn draw(&self) -> Result<(), gl::DrawError> {
        self.program
            .with_uniforms(self.camera.as_binding())
            .with_settings(
                gl::DrawSettings::new()
                    .with_clear_color([0.1, 0.2, 0.3, 1.0])
                    .with_clear_depth(1.0)
                    .with_depth_test(gl::Comparison::Less),
            )
            .draw(
                gl::VertexSpec::new(gl::PrimitiveMode::Triangles).with_vertex_data(VsInput {
                    instance: self.instances.as_binding().with_instancing(),
                    model_pos: self.arrow.as_binding(),
                }),
            )?;

        Ok(())
    }
}

impl Default for Camera<Gl> {
    fn default() -> Self {
        Self {
            world_to_view: glam::Mat4::IDENTITY.into(),
            view_to_screen: {
                let left: f32 = 0.;
                let right: f32 = SCREEN_SIZE as f32;
                let bottom: f32 = 0.;
                let top: f32 = SCREEN_SIZE as f32;
                let near: f32 = -1.;
                let far: f32 = 1.;
                glam::Mat4::orthographic_rh_gl(left, right, bottom, top, near, far).into()
            },
        }
    }
}
