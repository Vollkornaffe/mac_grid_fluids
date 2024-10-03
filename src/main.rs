use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use posh::{gl, Gl};
use render::{Graphics, Instance};

mod render;
mod shader;

const SCREEN_SIZE: u32 = 10;
const PIXEL_PER_UNIT: u32 = 100;

fn main() {
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();

    let gl_attr = video.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::GLES);
    gl_attr.set_context_version(3, 0);

    let window = video
        .window(
            "MAC Grid Fluid",
            SCREEN_SIZE * PIXEL_PER_UNIT,
            SCREEN_SIZE * PIXEL_PER_UNIT,
        )
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

const GRID_SIZE: usize = 10;

fn instances(_time: f32) -> Vec<Instance<Gl>> {
    (0..GRID_SIZE)
        .flat_map(|x| {
            (0..GRID_SIZE).map(move |y| {
                let x = x as f32 / GRID_SIZE as f32;
                let y = y as f32 / GRID_SIZE as f32;
                let world_pos = glam::vec3(x, y, 0.) * SCREEN_SIZE as f32 * 0.9;
                let model_to_view = glam::Mat4::from_translation(world_pos);
                let color = glam::vec3(x, y, 0.);

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
