use macroquad::{color::hsl_to_rgb, miniquad::window::screen_size, prelude::*};
use num_complex::Complex;
use std::f32::consts::PI;

// ! Change function at line 23 and 32

// Constants
const ZOOM_FACTOR: f32 = 1.1;
const SCROLL_FACTOR: f32 = 50.;
const K: i32 = 2;
const MAX_LIGHTNESS: f32 = 0.7;
const SATURATION: f32 = 1.;
const START_BOUNDARIES: f32 = 10.;

fn map_value(value: f32, from_min: f32, from_max: f32, to_min: f32, to_max: f32) -> f32 {
    // Normalize value to the range [0, 1]
    let normalized_value = (value - from_min) / (from_max - from_min);

    // Scale normalized value to the new range
    normalized_value * (to_max - to_min) + to_min
}

fn f_of_z(z : Complex<f32>) -> Complex<f32> {
    // fz = ((-1. / 2.) * (z * z)).exp();
    let fz = z * z * z - 1.;
    // fz = z / z.cos();
    // fz = z.sin() - 0.5;

    fz
}

fn riemann_zeta(z : Complex<f32>) -> Complex<f32> {
    let mut fz = Complex::new(0., 0.);

    // 50 first terms of the sum
    for i in 1..50 {
        fz += z.expf(i as f32).inv();
    }

    fz
}
#[macroquad::main("Complex function plotter")]
async fn main() {
    // Initialize window
    let (width, height) = screen_size();

    let mut image = Image::gen_image_color(width as u16, height as u16, WHITE);
    let mut texture = Texture2D::from_image(&image);

    let (mut w, mut h) = (image.width(), image.height());

    // Initial graph boundaries
    let mut boundary = START_BOUNDARIES;

    // Graph translation offsets
    let mut x_offset = 0.;
    let mut y_offset = 0.;

    loop {
        // Check for window resizing
        let (new_width, new_height) = screen_size();
        if new_width != width || new_height != height {
            image = Image::gen_image_color(new_width as u16, new_height as u16, WHITE);
            texture = Texture2D::from_image(&image);

            (w, h) = (image.width(), image.height());
        }

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

                z = f_of_z(z);

                // Map angle from radians to degrees
                let hue = map_value(z.arg(), -PI, PI, 0., 1.);

                let norm = z.norm();

                // |z|^K/(|z|^K+1)
                let z_k = norm.powi(K);
                let lightness = MAX_LIGHTNESS * z_k / (z_k + 1.);

                let color = hsl_to_rgb(hue, SATURATION, lightness);

                // Set according color
                image.set_pixel(x as u32, y as u32, color);
            }
        }

        texture.update(&image);

        draw_texture(&texture, 0., 0., WHITE);

        // Draw axes
        let x_axe_pos = map_value(
            0.,
            (-boundary) + x_offset,
            boundary + x_offset,
            0.,
            w as f32,
        );
        let y_axe_pos = map_value(
            0.,
            (-boundary) + y_offset,
            boundary + y_offset,
            0.,
            h as f32,
        );
        draw_line(x_axe_pos, 0., x_axe_pos, h as f32, 1., BLACK);
        draw_line(0., y_axe_pos, w as f32, y_axe_pos, 1., BLACK);

        // Get point value
        let (mut mx, mut my) = mouse_position();

        mx = map_value(mx, 0., w as f32, -boundary + x_offset, boundary + x_offset);
        my = map_value(my, 0., h as f32, -boundary + y_offset, boundary + y_offset);

        let mut z = Complex::new(mx,my);

        z = f_of_z(z);

        let (re, im) = (z.re, z.im);

        let z_text = &format!("z = {mx:.3} + {my:.3}i");
        let fz_text = &format!("f(z) = {re:.3} + {im:.3}i");


        draw_text(z_text, 10., 30., 30., BLACK);
        draw_text(fz_text, 10., 60., 30., BLACK);


        next_frame().await;
    }
}
