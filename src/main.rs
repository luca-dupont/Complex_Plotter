extern crate num_complex;
use macroquad::prelude::*;
use num_complex::Complex;
use std::f64::consts::PI;

const WIDTH: f32 = 750.0;
const HEIGHT: f32 = 750.0;

const W_OS_FACTOR: f32 = 2.0;
const H_OS_FACTOR: f32 = 1.925;

fn map_value(value: f32, from_min: f32, from_max: f32, to_min: f32, to_max: f32) -> f32 {
    // First, normalize the value to the range [0, 1]
    let normalized_value = (value - from_min) / (from_max - from_min);

    // Then, scale the normalized value to the new range
    normalized_value * (to_max - to_min) + to_min
}

fn angle_to_rgb(hue: f32) -> (f32, f32, f32) {
    let chroma = 1.0;
    let x = chroma * (1.0 - ((hue / 60.0) % 2.0 - 1.0).abs());
    let m = 1.0 - chroma / 2.0;

    let (r, g, b) = if hue < 60.0 {
        (chroma, x, 0.0)
    } else if hue < 120.0 {
        (x, chroma, 0.0)
    } else if hue < 180.0 {
        (0.0, chroma, x)
    } else if hue < 240.0 {
        (0.0, x, chroma)
    } else if hue < 300.0 {
        (x, 0.0, chroma)
    } else {
        (chroma, 0.0, x)
    };

    let r = r + m;
    let g = g + m;
    let b = b + m;

    (r, g, b)
}

#[macroquad::main("Complex function grapher")]
async fn main() {
    request_new_screen_size(WIDTH / W_OS_FACTOR, HEIGHT / H_OS_FACTOR);

    let mut image = Image::gen_image_color(WIDTH as u16, HEIGHT as u16, WHITE);

    let texture = Texture2D::from_image(&image);

    let mut boundary = 100.;

    loop {
        let w = image.width();
        let h = image.height();

        clear_background(BLACK);

        if is_key_down(KeyCode::Up) {
            boundary /= 1.1;
        } else if is_key_down(KeyCode::Down) {
            boundary *= 1.1;
        }

        for x in 0..w {
            for y in 0..h {
                let (a, b) = (
                    map_value(x as f32, 0., 750., -boundary, boundary),
                    map_value(y as f32, 0., 750., -boundary, boundary),
                );

                let mut z = Complex::new(a, b);

                z = z * z - 1.;

                // let norm = z.norm();

                let angle = map_value(z.arg(), -PI as f32, PI as f32, 0., 360.);

                let (r1, g1, b1) = angle_to_rgb(angle);

                let color = Color {
                    r: r1,
                    g: g1,
                    b: b1,
                    a: 0.8,
                };

                image.set_pixel(x as u32, y as u32, color);
            }
        }

        // Draw axes

        for x in 0..w {
            image.set_pixel(x as u32, (h / 2) as u32, BLACK);
        }
        for y in 0..h {
            image.set_pixel((w / 2) as u32, y as u32, BLACK);
        }

        texture.update(&image);

        draw_texture(texture, 0., 0., WHITE);

        next_frame().await;
    }
}
