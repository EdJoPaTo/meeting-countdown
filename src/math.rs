use bracket_color::prelude::HSV;

#[allow(clippy::cast_precision_loss)]
pub fn calc_relative_position(start: i64, end: i64, position: i64) -> f32 {
    let relative_max = end - start;
    let relative_position = position - start;
    relative_position as f32 / relative_max as f32
}

/// Converts from f32 Hue to u8 rgb values
/// * `hue` - Hue from 0.0 to 360.0
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
#[must_use]
pub fn hue_to_rgb(hue: u16) -> (u8, u8, u8) {
    let hsv = HSV::from_f32(f32::from(hue) / 360.0, 1.0, 1.0);
    let rgb = hsv.to_rgb();

    let red = (rgb.r * 255.0) as u8;
    let green = (rgb.g * 255.0) as u8;
    let blue = (rgb.b * 255.0) as u8;

    (red, green, blue)
}

#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss
)]
pub fn interpolate_u16(start: u16, end: u16, position: f32) -> u16 {
    let relative_max = f32::from(end) - f32::from(start);
    let relative_position = relative_max * position;
    (f32::from(start) + relative_position) as u16
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
fn hue_to_rgb_red() {
    assert_eq!((255, 0, 0), hue_to_rgb(0));
}

#[test]
fn hue_to_rgb_green() {
    assert_eq!((0, 255, 0), hue_to_rgb(120));
}

#[test]
fn hue_to_rgb_yellow() {
    assert_eq!((255, 255, 0), hue_to_rgb(60));
}

#[test]
fn interpolate_start() {
    assert_eq!(2, interpolate_u16(2, 4, 0.0));
}

#[test]
fn interpolate_end() {
    assert_eq!(4, interpolate_u16(2, 4, 1.0));
}

#[test]
fn interpolate_high_to_low() {
    assert_eq!(2, interpolate_u16(5, 1, 0.75));
}

#[test]
fn interpolate_half() {
    assert_eq!(3, interpolate_u16(2, 4, 0.5));
}
