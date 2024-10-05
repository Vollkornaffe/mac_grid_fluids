use std::{
    fs::File,
    io::{BufRead, BufReader},
    iter::once,
};

use glam::{uvec2, vec4, Vec2, Vec3, Vec4};
use posh::{gl, Gl};
use render::{Graphics, Instance};
use simulation::{Cell, Simulation};
use tracing::subscriber::set_global_default;
use tracing_subscriber::FmtSubscriber;

mod render;
mod shader;
mod simulation;

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;

fn main() {
    set_global_default(FmtSubscriber::default()).unwrap();

    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();

    let gl_attr = video.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::GLES);
    gl_attr.set_context_version(3, 0);

    let window = video
        .window("MAC Grid Fluid", WIDTH, HEIGHT)
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

    let grid_dimensions = uvec2(60, 30);
    let cell_size = 20.;
    let time_step = 0.5;
    let mut simulation = Simulation::new(grid_dimensions, cell_size, time_step);

    let cell_offset = Vec2::splat(2. * simulation.cell_size);
    let mut cursor_cell = Cell {
        position: Vec2::ZERO,
        velocity: Vec2::Y,
        color: Vec3::Y,
    };

    loop {
        for event in event_loop.poll_iter() {
            type E = sdl2::event::Event;

            match event {
                E::MouseMotion { x, y, .. } => {
                    cursor_cell.position.x = x as f32 - cell_offset.x;
                    cursor_cell.position.y = (HEIGHT as i32 - y) as f32 - cell_offset.y;
                }
                E::Quit { .. } => {
                    return;
                }
                _ => {}
            }
        }

        simulation.step();

        cursor_cell.velocity = simulation.interpolate_velocity(cursor_cell.position);

        let cell_to_instance = |cell: Cell| Instance::<Gl> {
            model_to_view: glam::Mat4::from_cols(
                cell.velocity.extend(0.).extend(0.) * simulation.cell_size,
                vec4(-cell.velocity.y, cell.velocity.x, 0., 0.).normalize_or_zero()
                    * simulation.cell_size,
                Vec4::Z,
                (cell_offset + cell.position).extend(0.).extend(1.),
            )
            .into(),
            color: cell.color.into(),
        };

        graphics.instances.set(
            &once(cursor_cell)
                .chain(simulation.cells())
                .map(cell_to_instance)
                .collect::<Vec<_>>(),
        );

        graphics.draw().unwrap();
        window.gl_swap_window();
    }
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
