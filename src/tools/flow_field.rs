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
    bounds: Rect,
    frequency: f64,
    amplitude: f64,
    step: f32,
    points: Vec<Point2>,
    stop: bool,
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
    pub fn new(x: f32, y: f32, bounds: Rect, noise: Box<dyn NoiseFn<[f64; 2]>>) -> Self {
        Particle {
            position: pt2(x, y),
            velocity: vec2(0.0, 0.0),
            acceleration: vec2(0.0, 0.0),
            bounds,
            noise,
            frequency: 0.001,
            amplitude: 6.0,
            step: 0.2,
            points: vec![],
            stop: false,
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

    fn step(&mut self, direction: Vec2, force: Vec2) {
        if !self.stop {
            let angle = self.noise.get([
                self.position.x as f64 * self.frequency,
                self.position.y as f64 * self.frequency,
            ]) * self.amplitude;

            // Update the volicty of the particle based on the direction
            let noise_force = vec2(
                rad_to_deg(angle.cos() as f32) * self.step,
                rad_to_deg(angle.sin() as f32) * self.step,
            );

            self.velocity += (noise_force + direction) * force;
            // Move particle
            self.position += self.velocity;

            // Apply damping to slow it down
            self.velocity *= self.acceleration;

            if self.bounds.contains(self.position) {
                self.points.push(self.position);
            } else {
                self.stop = true;
            }
        }
    }

    pub fn move_particle(&mut self) {
        self.step(vec2(0.0, 0.0), vec2(1.0, 1.0));
    }

    pub fn move_particle_with_direction(&mut self, direction: Vec2, force: Vec2) {
        self.step(direction, force)
    }

    pub fn draw_line<'a>(&self, draw: &'a Draw) -> Drawing<'a, Path> {
        draw.path()
            .stroke()
            .stroke_weight(10.0)
            .points(self.points.clone())
    }

    pub fn line(&self) -> &Vec<Point2> {
        &self.points
    }

    pub fn height(&self) -> f32 {
        let (max, min) = self.points.iter().map(|p| p.y).fold(
            (std::f32::MIN, std::f32::MAX),
            |acc, x| match acc {
                (max, min) if x > max && x < min => (x, x),
                (max, min) if x > max => (x, min),
                (max, min) if x < min => (max, x),
                _ => acc,
            },
        );
        max - min
    }
}
