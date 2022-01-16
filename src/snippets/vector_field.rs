fn view(app: &App, model: &Model, frame: Frame) {
  frame.clear(PURPLE);

  let draw = app.draw();
  let win = app.window_rect();

  let r = app.window_rect();

  for r in r.subdivisions_iter() {
      for r in r.subdivisions_iter() {
          for r in r.subdivisions_iter() {
              let point: [f64; 2] = [(r.x() / win.w()) as f64, (r.y() / win.h()) as f64];

              let rotation = model.perlin.get(point) as f32 * PI;
              let draw = draw.xy(r.xy()).rotate(rotation);
              // draw.line().start(vec2(-10.0 , -10.0)).end(vec2(10.0, 10.0));
              draw.line().start(vec2(0.0, 0.0) ).end(r.wh() / 2.0);
          }
      }
  }

  draw.to_frame(app, &frame).unwrap();
}
