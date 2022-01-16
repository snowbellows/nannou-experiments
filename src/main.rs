use experiments::inspirations::tyler_hobbs::{model, update, view};

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}
