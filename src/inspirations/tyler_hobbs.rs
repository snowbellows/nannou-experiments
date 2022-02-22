use itertools::Itertools;
use nannou::{
    noise::{NoiseFn, Perlin, Seedable},
    prelude::*,
    rand::{RngCore, SeedableRng},
};
use rand_chacha::ChaCha8Rng;

use crate::tools::{check_within_bounds, colour_gen::ColourIter, flow_field::Particle};

pub struct Model {
    particles: Option<Vec<Vec<(Particle, Particle)>>>,
    steps: u32,
    steps_taken: u32,
    window: WindowId,
    captured: bool,
    colour_vec: Vec<Hsva>,
}

const SEED: u32 = 23456;
const NUM_GROUPS: u32 = 2;
const NUM_PAIRS: u32 = 10;
const NUM_SECTIONS: u32 = 20;
const COLORS: [f32; 3] = [0.0, 150.0, 210.0];

const LIGHT_ROSE: (f32, f32, f32) = (4.0 / 360.0, 0.75, 0.87);
const TEAL: (f32, f32, f32) = (170.0 / 360.0, 0.55, 0.41);
const DARK_GREY: (f32, f32, f32) = (17.0 / 360.0, 0.19, 0.24);
const DENIM: (f32, f32, f32) = (210.0 / 360.0, 0.37, 0.39);
const RED: (f32, f32, f32) = (358.0 / 360.0, 0.58, 0.50);
const MAC_AND_CHEESE: (f32, f32, f32) = (40.0 / 360.0, 0.96, 0.61);

const COLOURS: [(f32, f32, f32); 6] = [LIGHT_ROSE, TEAL, DARK_GREY, DENIM, RED, MAC_AND_CHEESE];

fn generate_particles(
    window_rect: Rect,
    num_groups: u32,
    num_pairs: u32,
) -> Vec<Vec<(Particle, Particle)>> {
    let half_width = window_rect.w() / 2.0;

    (0..num_groups)
        .map(|g_i| {
            let particle_perlin = Perlin::new().set_seed(SEED + g_i as u32);
            println!("{}", g_i);
            (0..num_pairs)
                .map(|p_i| {
                    let mut rng = ChaCha8Rng::seed_from_u64(SEED as u64 + g_i as u64 + p_i as u64);

                    let x1 = dbg!(map_range(
                        rng.next_u64(),
                        u64::MIN,
                        u64::MAX,
                        // dbg!((window.w() / NUM_GROUPS as f32 * g_i as f32) - half_width),
                        // dbg!((window.w() / NUM_GROUPS as f32 * (g_i as f32 + 1.0)) - half_width),
                        (window_rect.w() / num_pairs as f32 * p_i as f32) - half_width - 10.0,
                        (window_rect.w() / num_pairs as f32 * (p_i as f32 + 1.0)) - half_width
                            + 10.0,
                    ));
                    let x2 = map_range(
                        rng.next_u64(),
                        u64::MIN,
                        u64::MAX,
                        x1 - (window_rect.w() / num_pairs as f32),
                        x1 + (window_rect.w() / num_pairs as f32),
                    );
                    let y = (-window_rect.h() / 2.0) + 20.0;

                    let bounds =
                        Rect::from_wh(vec2(window_rect.w() + 50.0, window_rect.h() - 20.0));
                    (
                        Particle::new(x1, y, bounds, Box::new(particle_perlin.clone()))
                            .set_step(0.1),
                        Particle::new(x2, y, bounds, Box::new(particle_perlin.clone()))
                            .set_step(0.1),
                    )
                })
                .collect::<Vec<(Particle, Particle)>>()
        })
        .collect()
}

pub fn model(app: &App) -> Model {
    let window = app.new_window().size(846, 1080).view(view).build().unwrap();

    let colour_vec = COLOURS
        .into_iter()
        .map(|(h, s, v)| hsva(h, s, v, 0.8))
        .collect::<Vec<Hsva>>();

    Model {
        particles: None,
        steps: 200,
        steps_taken: 0,
        window,
        captured: false,
        colour_vec,
    }
}

pub fn update(app: &App, model: &mut Model, _update: Update) {
    if let Some(particles) = &mut model.particles {
        let force = vec2(0.8, 1.0);
        let direction = vec2(0.0, 10.0);
        if model.steps_taken <= model.steps {
            for p in particles {
                for (p1, p2) in p {
                    p1.move_particle_with_direction(direction, force);
                    p2.move_particle_with_direction(direction, force);
                }
            }
            model.steps_taken += 1;
        } else if model.captured == false {
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

        let particles = generate_particles(window, NUM_GROUPS, NUM_PAIRS);

        model.particles = Some(particles);
    }
}

pub fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(WHITE);
    let draw = app.draw();
    if let Some(particles) = &model.particles {
        for (g_i, (p, c)) in (&particles).into_iter().zip(COLORS).enumerate() {
            for (p_i, (p1, p2)) in p.into_iter().enumerate() {
                println!("{}", p1.line().len());
                println!("{}", NUM_SECTIONS);

                let chunk_size = dbg!(p1.line().len() / NUM_SECTIONS as usize);
                if chunk_size > 0 {
                    let sections: Vec<Vec<Vec2>> = p1
                        .line()
                        // .into_iter()
                        // .sorted_by(|x, y| PartialOrd::partial_cmp(&x.y, &y.y).unwrap())
                        .chunks(chunk_size)
                        .into_iter()
                        .zip(
                            p2.line()
                                // .into_iter()
                                // .sorted_by(|x, y| PartialOrd::partial_cmp(&x.y, &y.y).unwrap())
                                .chunks(chunk_size)
                                .into_iter(),
                        )
                        .map(|(x,  y)| {
                            x.into_iter()
                                .chain( y.into_iter().rev())
                                .copied()
                                .collect::<Vec<Vec2>>()
                        })
                        .collect::<Vec<Vec<Vec2>>>();

                    let mut colour_iter = ColourIter::new(&model.colour_vec);
                    println!("{:?}", sections.len());
                    println!(
                        "{:?}",
                        sections.iter().map(|s| s.len()).collect::<Vec<usize>>()
                    );
                    let points: Vec<Vec2> = p1
                        .line()
                        .clone()
                        .into_iter()
                        .chain(p2.line().clone().into_iter().rev())
                        .collect();
                    draw.polygon()
                        .points(points)
                        // .color(*colour_iter.next().unwrap_or(&hsva(0.0, 0.0, 0.0, 0.0)));
                        .color(hsva(0.1, 0.8, 0.8, 0.8));

                    for (s_i, section) in sections.into_iter().enumerate() {
                        let hue = dbg!(s_i as f32 / NUM_SECTIONS as f32);
                        draw.polygon()
                            .points(section)
                            // .color(*colour_iter.next().unwrap_or(&hsva(0.0, 0.0, 0.0, 0.0)));
                            .color(hsva(hue, 0.8, 0.8, 0.8));
                    }
                }
            }
        }
    }
    draw.to_frame(app, &frame).unwrap();
}
