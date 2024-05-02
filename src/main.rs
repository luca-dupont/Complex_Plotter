extern crate num_complex;
use macroquad::prelude::*;
use num_complex::Complex;
use std::f32::consts::PI;

// ! Example func : z^3 - 1
// ! Change at line 128

const WIDTH: f32 = 750.;
const HEIGHT: f32 = 750.;

const ZOOM_FACTOR: f32 = 1.1;
const SCROLL_FACTOR: f32 = 50.;

const R: i32 = 2;

const W_OS_FACTOR: f32 = 2.;
const H_OS_FACTOR: f32 = 1.925;

const START_BOUNDARIES: f32 = 10.;

fn map_value(value: f32, from_min: f32, from_max: f32, to_min: f32, to_max: f32) -> f32 {
    // Normalize value to the range [0, 1]
    let normalized_value = (value - from_min) / (from_max - from_min);

    // Scale normalized value to the new range
    normalized_value * (to_max - to_min) + to_min
}

fn angle_to_rgb(hue: f32) -> (f32, f32, f32) {
    let chroma = 1.0;
    let x = chroma * (1.0 - ((hue / 60.0) % 2.0 - 1.0).abs());
    let m = 1.0 - chroma / 2.0;

    let (r, g, b) = match hue {
        _ if hue < 60.0 => (chroma, x, 0.0),
        _ if hue < 120.0 => (x, chroma, 0.0),
        _ if hue < 180.0 => (0.0, chroma, x),
        _ if hue < 240.0 => (0.0, x, chroma),
        _ if hue < 300.0 => (x, 0.0, chroma),
        _ => (chroma, 0.0, x),
    };

    (r + m, g + m, b + m)
}

#[macroquad::main("Complex function grapher")]
async fn main() {
    // Device specific resizing factor | Tweak as needed
    request_new_screen_size(WIDTH / W_OS_FACTOR, HEIGHT / H_OS_FACTOR);

    let mut image = Image::gen_image_color(WIDTH as u16, HEIGHT as u16, WHITE);

    let texture = Texture2D::from_image(&image);

    // Initial graph boundaries
    let mut boundary = START_BOUNDARIES;

    // Graph translation offsets
    let mut x_offset = 0.;
    let mut y_offset = 0.;

    loop {
        let w = image.width();
        let h = image.height();

        clear_background(BLACK);

        // Handle events
        if is_key_down(KeyCode::Equal) {
            // Zoom in
            boundary /= ZOOM_FACTOR;
        }
        if is_key_down(KeyCode::Minus) {
            // Zoom out
            boundary *= ZOOM_FACTOR;
        }
        if is_key_down(KeyCode::Right) {
            // Scroll right
            x_offset += boundary / SCROLL_FACTOR;
        }
        if is_key_down(KeyCode::Left) {
            // Scroll left
            x_offset -= boundary / SCROLL_FACTOR;
        }
        if is_key_down(KeyCode::Down) {
            // Scroll down
            y_offset += boundary / SCROLL_FACTOR;
        }
        if is_key_down(KeyCode::Up) {
            // Scroll up
            y_offset -= boundary / SCROLL_FACTOR;
        }

        // Calculate value for each pixel after function
        for x in 0..w {
            for y in 0..h {
                let (a, b) = (
                    // Map pixel to the graph boundaries
                    map_value(
                        x as f32,
                        0.,
                        w as f32,
                        (-boundary) + x_offset,
                        boundary + x_offset,
                    ),
                    map_value(
                        y as f32,
                        0.,
                        h as f32,
                        (-boundary) + y_offset,
                        boundary + y_offset,
                    ),
                );

                let mut z = Complex::new(a, b);

                // //* ↓ Implementation for Riemann Zeta function with the first 50 terms | *SLOW* ↓ */
                // let mut z2 = Complex::new(0., 0.);

                // for i in 1..50 {
                //     z2 += z.expf(i as f32).inv();
                // }

                // z = z2;

                // ! ↓↓ CHANGE FUNC HERE ↓↓
                // z = ((-1. / 2.) * (z * z)).exp();
                // z = z * z * z - 1.;
                z = z / z.cos();
                // z = z.sin() - 0.5;

                // Map angle from radians to degrees
                let angle = map_value(z.arg(), -PI, PI, 0., 360.);

                let norm = z.norm();

                // Map angle to color wheel color
                let (r1, g1, b1) = angle_to_rgb(angle);

                // |z|^R/(|z|^R+1)
                let z_r = norm.powi(R);
                let alpha = z_r / (z_r + 1.);

                let color = Color {
                    r: r1,
                    g: g1,
                    b: b1,
                    a: alpha,
                };

                // Set according color
                image.set_pixel(x as u32, y as u32, color);
            }
        }

        // Draw axes
        for x in 0..w {
            let y_pos = map_value(
                0.,
                (-boundary) + y_offset,
                boundary + y_offset,
                0.,
                h as f32,
            );
            if y_pos < h as f32 && y_pos > 0. {
                image.set_pixel(x as u32, y_pos as u32, BLACK);
            }
        }
        for y in 0..h {
            let x_pos = map_value(
                0.,
                (-boundary) + x_offset,
                boundary + x_offset,
                0.,
                w as f32,
            );
            if x_pos < w as f32 && x_pos > 0. {
                image.set_pixel(x_pos as u32, y as u32, BLACK);
            }
        }

        texture.update(&image);

        draw_texture(texture, 0., 0., WHITE);

        next_frame().await;
    }
}
