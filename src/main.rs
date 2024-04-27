use macroquad::prelude::*;

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

#[macroquad::main("Complex function grapher")]
async fn main() {
    request_new_screen_size(WIDTH / W_OS_FACTOR, HEIGHT / H_OS_FACTOR);

    let mut image = Image::gen_image_color(WIDTH as u16, HEIGHT as u16, WHITE);

    let texture = Texture2D::from_image(&image);

    println!("{:?}", (image.width(), image.height()));

    loop {
        let w = image.width();
        let h = image.height();

        clear_background(BLACK);

        for x in 0..w {
            for y in 0..h {
                let color = Color {
                    r: map_value(x as f32, 0., w as f32, 0., 255.),
                    // g: map_value(y as f32, 0., h as f32, 0., 255.),
                    g: 122.5,
                    b: map_value(y as f32, 0., h as f32, 0., 255.),
                    a: 1.,
                };
                image.set_pixel(x as u32, y as u32, color);
            }
        }

        texture.update(&image);

        draw_texture(texture, 0., 0., WHITE);

        next_frame().await;
    }
}
