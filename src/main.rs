use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use posh::{gl, Gl};
use render::{Graphics, Instance};

mod render;
mod shader;

const SCREEN_SIZE: u32 = 10;

fn main() {
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();

    let gl_attr = video.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::GLES);
    gl_attr.set_context_version(3, 0);

    let window = video
        .window("Teapot instancing", 1024, 768)
        .opengl()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    let gl = unsafe {
        glow::Context::from_loader_function(|s| video.gl_get_proc_address(s) as *const _)
    };
    let gl = gl::Context::new(gl).unwrap();
    let demo = Graphics::new(gl).unwrap();

    let mut event_loop = sdl.event_pump().unwrap();

    loop {
        for event in event_loop.poll_iter() {
            use sdl2::event::Event::*;

            if matches!(event, Quit { .. }) {
                return;
            }
        }

        demo.draw().unwrap();
        window.gl_swap_window();
    }
}

fn instances(_time: f32) -> Vec<Instance<Gl>> {
    (0..10)
        .flat_map(|x| {
            (0..10).map(move |y| {
                let world_pos = glam::uvec3(x, y, 0).as_vec3();
                let model_to_view = glam::Mat4::from_translation(world_pos);
                let color = glam::uvec3(x, 10 - y, 0).as_vec3() / 10.0;

                Instance {
                    model_to_view: model_to_view.into(),
                    color: color.into(),
                }
            })
        })
        .collect()
}

fn arrow_positions() -> Vec<gl::Vec3> {
    let file = File::open("arrow.csv").expect("Could not find arrow.csv");
    BufReader::new(file)
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let cols = line.split(",").collect::<Vec<_>>();
            assert_eq!(cols.len(), 3);

            [
                cols[0].parse().unwrap(),
                cols[1].parse().unwrap(),
                cols[2].parse().unwrap(),
            ]
            .into()
        })
        .collect()
}
