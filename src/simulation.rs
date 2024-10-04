use glam::{mat2, uvec2, vec2, IVec2, UVec2, Vec2, Vec3};
use rand::Rng;

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub position: Vec2,
    pub velocity: Vec2,
    pub color: Vec3,
}

pub struct Simulation {
    pub time_step: f32,
    pub cell_size: f32,
    pub dimensions: UVec2,
    pub pressures: Vec<f32>,
    pub velocities_x: Vec<f32>,
    pub velocities_y: Vec<f32>,
}

impl Simulation {
    pub fn new(dimensions: UVec2, cell_size: f32, time_step: f32) -> Self {
        assert!(dimensions.element_product() != 0);

        let mut rng = rand::thread_rng();
        let mut random_float = || rng.gen_range(-1.0..1.);

        let pressures = vec![0.; dimensions.element_product() as usize];

        let velocities_x_count = ((dimensions.x + 1) * dimensions.y) as usize;
        let velocities_y_count = (dimensions.x * (dimensions.y + 1)) as usize;

        let velocities_x = (0..velocities_x_count).map(|_| random_float()).collect();
        let velocities_y = (0..velocities_y_count).map(|_| random_float()).collect();

        Self {
            time_step,
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
        self.boundary();
        self.advect();
        self.project();
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

        let Vec2 { x: dx, y: dy } = normalized - reference.as_vec2() - 0.5 * Vec2::Y;

        assert!(dx >= 0.);
        assert!(dx <= 1.);
        assert!(dy >= 0.);
        assert!(dy <= 1.);

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

        let Vec2 { x: dx, y: dy } = normalized - reference.as_vec2() - 0.5 * Vec2::X;

        assert!(dx >= 0.);
        assert!(dx <= 1.);
        assert!(dy >= 0.);
        assert!(dy <= 1.);

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

    fn interpolate_velocity_with_normalized(&self, normalized: Vec2) -> Vec2 {
        vec2(
            self.interpolate_velocity_x(normalized),
            self.interpolate_velocity_y(normalized),
        )
    }

    fn advect(&mut self) {
        let velocities_x = self
            .velocities_x_iter()
            .map(|UVec2 { x, y }| {
                let normalized = vec2(x as f32, 0.5 + y as f32);
                let velocity = self.interpolate_velocity_with_normalized(normalized);
                let lookup = normalized - self.time_step * velocity / self.cell_size;
                self.interpolate_velocity_with_normalized(lookup).x
            })
            .collect();
        let velocities_y = self
            .velocities_y_iter()
            .map(|UVec2 { x, y }| {
                let normalized = vec2(0.5 + x as f32, y as f32);
                let velocity = self.interpolate_velocity_with_normalized(normalized);
                let lookup = normalized - self.time_step * velocity / self.cell_size;
                self.interpolate_velocity_with_normalized(lookup).y
            })
            .collect();

        self.velocities_x = velocities_x;
        self.velocities_y = velocities_y;
    }

    fn boundary(&mut self) {
        for i in 0..self.dimensions.x {
            let bot = self.velocities_y_idx(uvec2(i, 0));
            let top = self.velocities_y_idx(uvec2(i, self.dimensions.y));
            self.velocities_y[top] = 0.;
            self.velocities_y[bot] = 0.;
        }

        for j in 0..self.dimensions.y {
            let left = self.velocities_x_idx(uvec2(0, j));
            let right = self.velocities_x_idx(uvec2(self.dimensions.x, j));
            self.velocities_x[left] = 0.;
            self.velocities_x[right] = 0.;
        }
    }

    fn cell_iter(&self) -> impl Iterator<Item = UVec2> + '_ {
        (0..self.dimensions.y).flat_map(|j| (0..self.dimensions.x).map(move |i| uvec2(i, j)))
    }

    fn velocities_x_iter(&self) -> impl Iterator<Item = UVec2> + '_ {
        (0..self.dimensions.y).flat_map(|j| (0..=self.dimensions.x).map(move |i| uvec2(i, j)))
    }

    fn velocities_y_iter(&self) -> impl Iterator<Item = UVec2> + '_ {
        (0..self.dimensions.x).flat_map(|i| (0..=self.dimensions.y).map(move |j| uvec2(i, j)))
    }

    fn project(&mut self) {}
}
