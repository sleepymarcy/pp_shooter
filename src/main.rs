pub mod engine;
pub mod entities;

use console_engine::{Color, KeyCode};
use engine::{pixel::Pixel, vector2f::Vec2f};
use entities::{actor::*, model::*, *};

fn main() {
    let mut screen = engine::MyEngine::init(30, 15, 15);

    let px_list = vec![
        Pixel::new(Color::Magenta, Vec2f::inew(0, 0)),
        Pixel::new(Color::Magenta, Vec2f::inew(1, 0)),
        Pixel::new(Color::Magenta, Vec2f::inew(1, 1)),
        Pixel::new(Color::Magenta, Vec2f::inew(2, 1)),
        Pixel::new(Color::Magenta, Vec2f::inew(2, 2)),
    ];

    // let center = Vec2f::new(
    //     screen.get_width() as f32 / 4.0,
    //     screen.get_height() as f32 / 2.0,
    // );

    let a = Actor::from_model(Model { pixels: px_list });

    loop {
        screen.clear();

        a.add_to_frame(&mut screen);

        // screen.set_pixel(Vec2f::inew(1, 1), Pixel::from_color(Color::Magenta));
        // screen.set_pixel(Vec2f::inew(2, 1), Pixel::from_color(Color::Magenta));

        // for px in &px_list {
        //     screen.set_pixel_2(center, px);
        // }

        screen.update();

        screen.wait_frame();

        if screen.key(KeyCode::Esc) {
            break;
        }
    }
}
