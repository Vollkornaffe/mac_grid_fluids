use glam::{vec2, UVec2, Vec2};

#[derive(Debug)]
pub struct Cell {
    pub position: Vec2,
    pub velocity: Vec2,
}

pub struct Simulation {
    pub cell_size: f32,
    pub dimensions: UVec2,
    pub pressures: Vec<f32>,
    pub velocities_x: Vec<f32>,
    pub velocities_y: Vec<f32>,
}

impl Simulation {
    pub fn new(dimensions: UVec2, cell_size: f32) -> Self {
        assert!(dimensions.element_product() != 0);

        let pressures = vec![0.; dimensions.element_product() as usize];
        let velocities_x = vec![1.; ((dimensions.x + 1) * dimensions.y) as usize];
        let velocities_y = vec![0.; (dimensions.x * (dimensions.y + 1)) as usize];

        Self {
            dimensions,
            cell_size,
            pressures,
            velocities_x,
            velocities_y,
        }
    }

    pub fn cells(&self) -> impl Iterator<Item = Cell> + '_ {
        (0..self.dimensions.y as usize).flat_map(move |j| {
            (0..self.dimensions.x as usize).map(move |i| {
                let position = vec2(i as f32 + 0.5, j as f32 + 0.5) * self.cell_size;
                let idx_x = i + j * (self.dimensions.x as usize + 1);
                let idx_y = i * (self.dimensions.y as usize + 1) + j;
                let velocity = 0.5
                    * vec2(
                        self.velocities_x[idx_x] + self.velocities_x[idx_x + 1],
                        self.velocities_y[idx_y] + self.velocities_y[idx_y + 1],
                    );
                Cell { position, velocity }
            })
        })
    }

    pub fn step(&mut self) {
        self.advect();
        self.project();
    }

    fn advect(&mut self) {}
    fn project(&mut self) {}
}
