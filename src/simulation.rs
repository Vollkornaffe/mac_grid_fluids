use glam::{mat2, vec2, IVec2, UVec2, Vec2, Vec3};
use rand::thread_rng;
use rand::Rng;

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub position: Vec2,
    pub velocity: Vec2,
    pub color: Vec3,
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

        let mut rng = rand::thread_rng();
        let mut random_float = || rng.gen_range(-1.0..1.);

        let pressures = vec![0.; dimensions.element_product() as usize];

        let velocities_x_count = ((dimensions.x + 1) * dimensions.y) as usize;
        let velocities_y_count = (dimensions.x * (dimensions.y + 1)) as usize;

        let velocities_x = (0..velocities_x_count).map(|_| random_float()).collect();
        let velocities_y = (0..velocities_y_count).map(|_| random_float()).collect();

        Self {
            dimensions,
            cell_size,
            pressures,
            velocities_x,
            velocities_y,
        }
    }

    pub fn cells(&self) -> impl Iterator<Item = Cell> + '_ {
        (0..self.dimensions.y).flat_map(move |j| {
            (0..self.dimensions.x).map(move |i| {
                let position = vec2(i as f32 + 0.5, j as f32 + 0.5) * self.cell_size;
                let velocity = self.interpolate_velocity(position);
                let color = Vec3::X;
                Cell {
                    position,
                    velocity,
                    color,
                }
            })
        })
    }

    pub fn step(&mut self) {
        self.advect();
        self.project();
    }

    fn cell_idx(&self, clamped: UVec2) -> usize {
        assert!(clamped.x < self.dimensions.x);
        assert!(clamped.y < self.dimensions.y);
        (clamped.x + clamped.y * self.dimensions.x) as usize
    }

    fn velocities_x_idx(&self, clamped: UVec2) -> usize {
        assert!(clamped.x < self.dimensions.x + 1);
        assert!(clamped.y < self.dimensions.y);
        (clamped.x + clamped.y * (self.dimensions.x + 1)) as usize
    }

    fn velocities_y_idx(&self, clamped: UVec2) -> usize {
        assert!(clamped.x < self.dimensions.x);
        assert!(clamped.y < self.dimensions.y + 1);
        (clamped.x * (self.dimensions.y + 1) + clamped.y) as usize
    }

    fn velocities_x_dimensions(&self) -> UVec2 {
        self.dimensions + UVec2::X
    }

    fn velocities_y_dimensions(&self) -> UVec2 {
        self.dimensions + UVec2::Y
    }

    fn velocity_x(&self, normalized: IVec2) -> f32 {
        let clamped = normalized
            .max(IVec2::ZERO)
            .as_uvec2()
            .min(self.velocities_x_dimensions() - UVec2::ONE);
        self.velocities_x[self.velocities_x_idx(clamped)]
    }

    fn velocity_y(&self, normalized: IVec2) -> f32 {
        let clamped = normalized
            .max(IVec2::ZERO)
            .as_uvec2()
            .min(self.velocities_y_dimensions() - UVec2::ONE);
        self.velocities_y[self.velocities_y_idx(clamped)]
    }

    fn interpolate_velocity_x(&self, normalized: Vec2) -> f32 {
        let shifted = normalized - 0.5 * Vec2::Y;
        let reference = shifted.floor().as_ivec2();

        let Vec2 { x: dx, y: dy } = normalized - reference.as_vec2();

        vec2(1. - dx, dx).dot(
            mat2(
                vec2(
                    self.velocity_x(reference),
                    self.velocity_x(reference + IVec2::X),
                ),
                vec2(
                    self.velocity_x(reference + IVec2::Y),
                    self.velocity_x(reference + IVec2::ONE),
                ),
            ) * vec2(1. - dy, dy),
        )
    }

    fn interpolate_velocity_y(&self, normalized: Vec2) -> f32 {
        let shifted = normalized - 0.5 * Vec2::X;
        let reference = shifted.floor().as_ivec2();

        let Vec2 { x: dx, y: dy } = normalized - reference.as_vec2();

        vec2(1. - dx, dx).dot(
            mat2(
                vec2(
                    self.velocity_y(reference),
                    self.velocity_y(reference + IVec2::X),
                ),
                vec2(
                    self.velocity_y(reference + IVec2::Y),
                    self.velocity_y(reference + IVec2::ONE),
                ),
            ) * vec2(1. - dy, dy),
        )
    }

    pub fn interpolate_velocity(&self, position: Vec2) -> Vec2 {
        let normalized = position / self.cell_size;
        vec2(
            self.interpolate_velocity_x(normalized),
            self.interpolate_velocity_y(normalized),
        )
    }

    fn advect(&mut self) {}
    fn project(&mut self) {}
}
