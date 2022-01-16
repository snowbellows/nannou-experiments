use nannou::{
    noise::{Perlin, Seedable},
    prelude::*,
};

use crate::tools::flow_field::Particle;

pub struct Model {
    particles: Vec<Particle>,
    steps: u32,
    steps_taken: u32,
}

pub fn model(app: &App) -> Model {
    const NUM_PARTICLES: u32 = 100;

    let perlin = Perlin::new().set_seed(42000);

    let particles = (1..NUM_PARTICLES)
        .map(|_| {
            let mut p = Particle::new(
                random_range(-app.window_rect().w() / 2.0, app.window_rect().w() / 2.0),
                random_range(-app.window_rect().h() / 2.0, app.window_rect().h() / 2.0),
                Box::new(perlin),
            );

            println!("{:?}", p);

            p.set_amplitude(6.0);
            p.set_frequency(0.001);
            p.set_step(0.2);
            p
        })
        .collect();

    Model {
        particles,
        steps: 60,
        steps_taken: 0,
    }
}

pub fn update(_app: &App, model: &mut Model, _update: Update) {
    if model.steps_taken <= model.steps {
        for p in &mut model.particles {
            p.move_particle();
        }
        model.steps_taken += 1;
    }
}

pub fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(WHITE);
    let draw = app.draw();

    for p in &model.particles {
        p.draw_line(&draw).color(BLACK);
    }

    draw.to_frame(app, &frame).unwrap();
}
