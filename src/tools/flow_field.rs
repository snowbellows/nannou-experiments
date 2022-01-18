use core::fmt;

use nannou::{
    draw::{primitive::Path, Drawing},
    noise::NoiseFn,
    prelude::*,
};

pub struct Particle {
    pub position: Point2,
    velocity: Vec2,
    acceleration: Vec2,
    noise: Box<dyn NoiseFn<[f64; 2]>>,
    frequency: f64,
    amplitude: f64,
    step: f32,
    points: Vec<Point2>,
}

impl fmt::Debug for Particle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Particle")
            .field("position", &self.position)
            .field("velocity", &self.velocity)
            .field("acceleration", &self.acceleration)
            .field("frequency", &self.frequency)
            .field("amplitude", &self.amplitude)
            .field("step", &self.step)
            .field("points", &self.points)
            .finish()
    }
}

impl Particle {
    pub fn new(x: f32, y: f32, noise: Box<dyn NoiseFn<[f64; 2]>>) -> Self {
        Particle {
            position: pt2(x, y),
            velocity: vec2(0.0, 0.0),
            acceleration: vec2(0.0, 0.0),
            noise,
            frequency: 0.001,
            amplitude: 6.0,
            step: 0.2,
            points: vec![],
        }
    }

    pub fn set_frequency(mut self, f: f64) -> Self {
        self.frequency = f;
        self
    }

    pub fn set_amplitude(mut self, a: f64) -> Self {
        self.amplitude = a;
        self
    }

    pub fn set_step(mut self, s: f32) -> Self {
        self.step = s;
        self
    }

    pub fn move_particle(&mut self) {
        let angle = self.noise.get([
            self.position.x as f64 * self.frequency,
            self.position.y as f64 * self.frequency,
        ]) * self.amplitude;

        // Update the volicty of the particle based on the direction
        // self.velocity += vec2(
        //     angle.cos() as f32,
        //     angle.sin() as f32,
        // );

        self.velocity += vec2(
            rad_to_deg(angle.cos() as f32) * self.step,
            rad_to_deg(angle.sin() as f32) * self.step,
        );

        // Move particle
        self.position += self.velocity;

        // Apply damping to slow it down
        self.velocity *= self.acceleration;

        self.points.push(self.position);
    }

    pub fn draw_line<'a>(&self, draw: &'a Draw) -> Drawing<'a, Path> {
        draw.path().stroke().points(self.points.clone())
    }
}
