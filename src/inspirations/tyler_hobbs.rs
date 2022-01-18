use itertools::Itertools;
use nannou::{
    noise::{Perlin, Seedable},
    prelude::*,
};

use crate::tools::flow_field::Particle;

pub struct Model {
    particles: Option<Vec<Vec<Vec<Particle>>>>,
    steps: u32,
    steps_taken: u32,
    window: WindowId,
    captured: bool
}

const SEED: u32 = 83;
const NUM_GROUPS: u32 = 3;
const NUM_PARTICLES: u32 = 20;
const COLORS: [f32; 3] = [0.0, 150.0, 210.0];

pub fn model(app: &App) -> Model {
    let window = app.new_window().size(1920, 1080).fullscreen().view(view).build().unwrap();

    Model {
        particles: None,
        steps: 100,
        steps_taken: 0,
        window,
        captured: false,
    }
}

pub fn update(app: &App, model: &mut Model, _update: Update) {
    if let Some(particles) = &mut model.particles {
        let force = vec2(0.8, 1.0);
        let direction = vec2(0.0, 10.0);
        if model.steps_taken <= model.steps {
            for p in particles {
                for p in p {
                    for p in p {
                        p.move_particle_with_direction(direction, force);
                    }
                    // p.into_iter().for_each(|p| p.move_particle_with_direction(direction, force));
                    // p.move_particle();
                }
            }
            model.steps_taken += 1;
        } else if model.captured == false{
            let image_name = format!("inspirations-th {} {}", SEED, chrono::Local::now());
            app.window(model.window).unwrap().capture_frame(
                app.project_path()
                    .unwrap()
                    .join("output")
                    .join(image_name)
                    .with_extension("png"),
            );
            model.captured = true
        }
    } else {
        let window = app.window_rect();

        let half_width = window.w() / 2.0;

        let particles: Vec<Vec<Vec<Particle>>> = (0..NUM_GROUPS)
            .map(|g_i| {
                (0..NUM_PARTICLES)
                    .map(|p_i| {
                        Particle::new(
                            map_range(
                                random_range(-half_width, half_width),
                                -half_width,
                                half_width,
                                (window.w() / NUM_PARTICLES as f32 * p_i as f32) - half_width,
                                (window.w() / NUM_PARTICLES as f32 * p_i as f32 + 1.0) - half_width,
                            ),
                            (-window.h() / 2.0) + 20.0,
                            Rect::from_wh(vec2(window.w(), window.h() - 20.0)),
                            Box::new(Perlin::new().set_seed(SEED + g_i)),
                        )
                        // .set_amplitude(6.0)
                        // .set_frequency(0.001)
                        .set_step(0.1)
                    })
                    .sorted_by(|p1, p2| p1.position.x.partial_cmp(&p2.position.x).unwrap())
                    .chunks(2)
                    .into_iter()
                    .map(|ps| ps.collect::<Vec<Particle>>())
                    .collect::<Vec<Vec<Particle>>>()
            })
            .collect();

            model.particles = Some(particles);
    }
}

pub fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(LIGHTGREY);
    let draw = app.draw();
    if let Some(particles) = &model.particles {
        for (g_i, (p, c)) in (&particles).into_iter().zip(COLORS).enumerate() {
            for (p_i, p) in p.into_iter().enumerate() {
                let points: Vec<Vec2> = p
                    .iter()
                    .map(|p| p.line().clone())
                    .enumerate()
                    .map(|(l_i, mut l)| {
                        if l_i % 2 == 0 {
                            l.reverse()
                        }
                        l
                    })
                    .flatten()
                    .collect();

                draw.polygon().points(points).color(Hsva::new(
                    (g_i as f32) * 50.0 + c + p_i as f32,
                    0.7,
                    0.8,
                    0.8,
                ));
            }
        }
    }
    draw.to_frame(app, &frame).unwrap();
}
