use glam::{Mat2, Vec2};

use crate::{GRID_SIZE, MARGIN, SCREEN_SIZE};

pub struct Cell {
    pub position: Vec2,
    pub velocity: Vec2,
}

pub struct Simulation {
    pub cells: Vec<Cell>,
}

impl Simulation {
    pub fn new() -> Self {
        Self {
            cells: (0..=GRID_SIZE)
                .flat_map(|x| {
                    (0..=GRID_SIZE).map(move |y| {
                        let x = x as f32 / GRID_SIZE as f32;
                        let y = y as f32 / GRID_SIZE as f32;
                        let position = Vec2::splat(SCREEN_SIZE as f32 * MARGIN / 2.)
                            + glam::vec2(x, y) * SCREEN_SIZE as f32 * (1. - MARGIN);
                        let velocity = Vec2::X;
                        Cell { position, velocity }
                    })
                })
                .collect(),
        }
    }

    pub fn step(&mut self) {
        for cell in &mut self.cells {
            cell.velocity = Mat2::from_angle(0.001) * cell.velocity;
        }
    }
}
