use posh::{sl, Sl};

use crate::render::{Camera, VsInput};

pub fn vertex_shader(camera: Camera<Sl>, vertex: VsInput<Sl>) -> sl::VsOutput<sl::Vec3> {
    sl::VsOutput {
        clip_position: camera.view_to_screen
            * camera.world_to_view
            * vertex.instance.model_to_view
            * vertex.model_pos.extend(1.0),
        interpolant: vertex.instance.color,
    }
}

pub fn fragment_shader(color: sl::Vec3) -> sl::Vec4 {
    color.extend(1.0)
}
