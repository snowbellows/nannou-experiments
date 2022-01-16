use nannou::{
  noise::NoiseFn,
  noise::{Perlin, Seedable},
  prelude::*,
};

pub struct Model {
  perlin: Perlin,
}

pub fn model(_app: &App) -> Model {
  Model {
      perlin: Perlin::new().set_seed(42),
  }
}

pub fn update(_app: &App, _model: &mut Model, _update: Update) {}

pub fn view(app: &App, model: &Model, frame: Frame) {
  frame.clear(PURPLE);

  let draw = app.draw();
  let win = app.window_rect();

  let r = app.window_rect();

  for r in r.subdivisions_iter() {
      for r in r.subdivisions_iter() {
          for r in r.subdivisions_iter() {
              let point: [f64; 2] = [(r.x() / win.w()) as f64, (r.y() / win.h()) as f64];

              let radius = model.perlin.get(point) as f32 * win.w() / 3.0;
              draw.xy(r.xy());
              // draw.rect()
              //     .color(Hsv::new(100.0, 1.0, 1.0))
              //     .xy(r.xy())
              //     .wh(r.wh());
              draw.ellipse()
                  .no_fill()
                  .stroke_weight(3.0)
                  .stroke(Hsv::new(0.0, 0.0, 1.0))
                  // .hsv(1.0, 1.0, 1.0)
                  // .xy(r.xy())
                  .radius(radius);
              // .finish();
          }
      }
  }

  draw.to_frame(app, &frame).unwrap();
}