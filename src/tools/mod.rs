use nannou::winit::platform::unix::x11::ffi::Bool;

pub mod colour_gen;
pub mod flow_field;

pub fn check_within_bounds<T>(start: T, end: T, value: T) -> bool
where
    T: PartialOrd,
{
    value >= start && value <= end
}
