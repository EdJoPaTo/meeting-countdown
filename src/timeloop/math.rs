#[allow(clippy::cast_precision_loss)]
pub fn calc_relative_position(start: i64, end: i64, position: i64) -> f64 {
    let relative_max = end - start;
    let relative_position = position - start;
    relative_position as f64 / relative_max as f64
}

#[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
pub fn interpolate(start: i64, end: i64, position: f64) -> i64 {
    let relative_max = end - start;
    let relative_position = relative_max as f64 * position;
    start + relative_position as i64
}

#[test]
fn calc_relative_position_start() {
    float_eq::assert_float_eq!(0.0, calc_relative_position(2, 4, 2), abs <= 0.1);
}
#[test]
fn calc_relative_position_end() {
    float_eq::assert_float_eq!(1.0, calc_relative_position(2, 4, 4), abs <= 0.1);
}
#[test]
fn calc_relative_position_half() {
    float_eq::assert_float_eq!(0.5, calc_relative_position(2, 4, 3), abs <= 0.1);
}

#[test]
fn interpolate_start() {
    assert_eq!(2, interpolate(2, 4, 0.0));
}

#[test]
fn interpolate_end() {
    assert_eq!(4, interpolate(2, 4, 1.0));
}

#[test]
fn interpolate_half() {
    assert_eq!(3, interpolate(2, 4, 0.5));
}
