use nannou::{
  prelude::*,
};



pub struct Model {
    recursions: i32,
    values: Vec<f32>,
}

pub fn model(_app: &App) -> Model {
    let recursions = 3;
    let squares = 4 ^ recursions;
    let values: Vec<f32> = (1..squares).map(|_| random_range(0.1, 1.0)).collect();
    Model { recursions, values }
}

pub fn update(_app: &App, _model: &mut Model, _update: Update) {}

pub fn view(app: &App, model: &Model, frame: Frame) {
        app.set_loop_mode(LoopMode::NTimes{ number_of_updates: 1});

    frame.clear(PURPLE);

    let draw = app.draw();
    let r = app.window_rect();
    let rects = get_subdivisions([r].to_vec(), model.recursions);

    for (r, _v) in rects.iter().zip(model.values.iter()) {
        draw.rect()
            .color(Hsv::new(100.0, 1.0, 1.0))
            .xy(r.xy())
            .wh(r.wh());
    }

    draw.to_frame(app, &frame).unwrap();
}

fn get_subdivisions(rects: Vec<Rect>, recursions: i32) -> Vec<Rect> {
    match recursions {
        0 => rects,
        _ => {
            let new_rects = rects
                .into_iter()
                .map(|r| {
                    let new_r = r.subdivisions();
                    println!("{:?}", new_r);
                    new_r
                })
                .flatten()
                .collect();
            get_subdivisions(new_rects, recursions - 1)
        }
    }
}
