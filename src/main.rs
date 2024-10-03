use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use glam::{vec4, Vec3, Vec4};
use posh::{gl, Gl};
use render::{Graphics, Instance};
use simulation::Simulation;

mod render;
mod shader;
mod simulation;

const SCREEN_SIZE: u32 = 100;
const PIXEL_PER_UNIT: u32 = 10;
const MARGIN: f32 = 0.2;

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
    let graphics = Graphics::new(gl).unwrap();

    let mut event_loop = sdl.event_pump().unwrap();
    let mut simulation = Simulation::new();

    loop {
        for event in event_loop.poll_iter() {
            use sdl2::event::Event::*;

            if matches!(event, Quit { .. }) {
                return;
            }
        }

        simulation.step();

        graphics.instances.set(
            &simulation
                .cells
                .iter()
                .map(|cell| Instance::<Gl> {
                    model_to_view: glam::Mat4::from_cols(
                        cell.velocity.extend(0.).extend(0.),
                        vec4(-cell.velocity.y, cell.velocity.x, 0., 0.).normalize_or_zero(),
                        Vec4::Z,
                        cell.position.extend(0.).extend(1.),
                    )
                    .into(),
                    color: Vec3::X.into(),
                })
                .collect::<Vec<_>>(),
        );

        graphics.draw().unwrap();
        window.gl_swap_window();
    }
}

const GRID_SIZE: usize = 25;

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
