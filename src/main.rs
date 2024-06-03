use macroquad::{color::hsl_to_rgb, miniquad::window::screen_size, prelude::*};
use num_complex::Complex;
use std::f32::consts::PI;
use rayon::prelude::*;

// ! Change function at line 28

// Constants
const ZOOM_FACTOR: f32 = 1.1;
const SCROLL_FACTOR: f32 = 50.;
const INF : f32 = f32::INFINITY;
const K: i32 = 2;
const MAX_LIGHTNESS: f32 = 0.7;
const SATURATION: f32 = 1.;
const START_BOUNDARIES: f32 = 10.;
const NUM_DIVISIONS : i32 = 4;
const TRANSPARENT_GREY: Color = Color{r : 220., g : 220., b : 220., a : 0.2};
const ZERO_THRESHOLD : f32 = 0.1e-30;

fn map_value(value: f32, from_min: f32, from_max: f32, to_min: f32, to_max: f32) -> f32 {
    // Normalize value to the range [0, 1]
    let normalized_value = (value - from_min) / (from_max - from_min);

    // Scale normalized value to the new range
    normalized_value * (to_max - to_min) + to_min
}

fn f_of_z(z : Complex<f32>) -> Complex<f32> {
    (z * z * z - 100.) / (z.powi(2) + 40.)
    // ((-1. / 2.) * (z * z)).exp()
    // z / z.cos()
    // z.sin() - 0.5

    // ↓↓↓ SLOW Riemann zeta function ↓↓↓ (Tip : If you really want to use it, make the screen small, so it runs at a reasonable speed)

    /*
    let mut fz = Complex::new(0., 0.);

    for i in 1..100 {
        let exp_z = z.expf(i as f32);
        if exp_z.re.abs() < ZERO_THRESHOLD && exp_z.im.abs() < ZERO_THRESHOLD {
            return Complex::new((exp_z.re as f32).signum()*INF,(exp_z.im as f32).signum()*INF)
        } else if exp_z.is_finite() {
            fz += exp_z.inv();
        }
    }
    fz
    */
}

#[macroquad::main("Complex function plotter")]
async fn main() {
    // Initialize window and image
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

        // Handle key presses
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

        // Make a vector of the pixel point pairs
        let x_y_vec : Vec<(u32, u32)> = (0..w as u32)
            .flat_map(|x| (0..h as u32).map(move |y| (x, y)))
            .collect();
        // Map the pairs from the width and height of screen to the boundaries of the graph and assign them to a complex number
        let complex_x_y_vec : Vec<Complex<f32>> = x_y_vec.clone()
            .into_iter()
            .map(|(x,y)| Complex::new(map_value(x as f32, 0., w as f32, (-boundary) + x_offset, boundary + x_offset, ),map_value(y as f32, 0., h as f32, boundary - y_offset, -boundary - y_offset, )))
            .collect();
        // Compute the result of the complex function with threads
        let fz: Vec<Complex<f32>> = complex_x_y_vec
            .par_iter()
            .map(|&z| f_of_z(z))
            .collect();

        // Loop through each point on the screen and it's complex result
        for ((x,y),z) in x_y_vec.into_iter().zip(fz.into_iter()) {

            // Map angle from radians to [0,1]
            let hue = map_value(z.arg(), -PI, PI, 0., 1.);

            let norm = z.norm();

            // Map norm to a lightness according to the function ||z||^K/(||z||^K+1) (Stereographic projection onto the Riemann Sphere with K = 2)
            let z_k = norm.powi(K);
            // Check if the value will be infinity, if so then make it very light, else let it do the function
            let lightness = if z_k != INF { MAX_LIGHTNESS * z_k / (z_k + 1.) } else {MAX_LIGHTNESS+0.1};

            // Get color according to norm and angle
            let color = hsl_to_rgb(hue, SATURATION, lightness);

            // Set according color
            image.set_pixel(x, y, color);
        }

        // Draw result on screen
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
            boundary - y_offset,
            -boundary - y_offset,
            0.,
            h as f32,
        );
        draw_line(x_axe_pos, 0., x_axe_pos, h as f32, 1., BLACK);
        draw_line(0., y_axe_pos, w as f32, y_axe_pos, 1., BLACK);

        // Get value at mouse position
        let (mut mx, mut my) = mouse_position();

        // Check if cursor is on screen, else put NaN
        if (0. ..w as f32).contains(&mx) && (0. ..h as f32).contains(&my) {
            mx = map_value(mx, 0., w as f32, -boundary + x_offset, boundary + x_offset);
            my = map_value(my, 0., h as f32, boundary - y_offset, -boundary - y_offset);
        } else {
            mx = f32::NAN;
            my = f32::NAN;
        }

        let z = Complex::new(mx,my);

        let res = f_of_z(z);

        let (re, im) = (res.re, res.im);

        // Output z and f(z) on the screen at mouse position
        let z_text = &format!("z = {mx:.3} + {my:.3}i");
        let fz_text = &format!("f(z) = {re:.3} + {im:.3}i");

        draw_rectangle(0.,0.,400., 75., TRANSPARENT_GREY);
        draw_text(z_text, 10., 30., 30., BLACK);
        draw_text(fz_text, 10., 60., 30., BLACK);

        // Draw axes divisions and values
        let x_division_spacing = w as f32 / NUM_DIVISIONS as f32;
        let y_division_spacing = h as f32 / NUM_DIVISIONS as f32;

        for i in 0..=NUM_DIVISIONS {
            let y = i as f32 * y_division_spacing;
            let x = i as f32 * x_division_spacing;

            // Draw labels
            let x_value = map_value(x, 0., w as f32, -boundary + x_offset, boundary + x_offset);
            let y_value = map_value(y, 0., h as f32, boundary - y_offset, -boundary - y_offset);

            draw_text(&format!("{:.1}", x_value), x-8., y_axe_pos - 10., 20., BLACK);
            draw_line(x, y_axe_pos - 5., x, y_axe_pos + 5., 2., BLACK);

            draw_text(&format!("{:.1}", y_value), x_axe_pos + 10., y+4., 20., BLACK);
            draw_line(x_axe_pos - 5., y, x_axe_pos+5., y, 2., BLACK);
        }

        next_frame().await;
    }
}
