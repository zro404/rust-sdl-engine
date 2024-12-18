use std::{path::Path, time::Duration};

use sdl2::{
    event::Event,
    image::{InitFlag, LoadTexture},
    keyboard::Keycode,
    pixels::Color,
    rect::{Point, Rect},
    render::{Texture, WindowCanvas},
};

// #[derive(Debug)]
struct CharacterBody<'a> {
    texture: Texture<'a>,
    sprite: Rect,
    position: Point,
    speed: i32,
}

fn render(canvas: &mut WindowCanvas, body: &CharacterBody) -> Result<(), String> {
    let (width, height) = canvas.output_size()?;
    // Treat the center of the screen as the (0, 0) coordinate
    let screen_position = body.position + Point::new(width as i32 / 2, height as i32 / 2);
    let screen_rect = Rect::from_center(screen_position, body.sprite.width(), body.sprite.height());

    canvas.copy(&body.texture, body.sprite, screen_rect)?;

    Ok(())
}

fn main() -> Result<(), String> {
    let ctx = sdl2::init()?;
    let video_subsystem = ctx.video()?;
    let _image_ctx = sdl2::image::init(InitFlag::PNG | InitFlag::JPG);

    let window = video_subsystem
        .window("rust sdl engine", 1280, 720)
        .position_centered()
        .build()
        .expect("Could not initialize video subsystem");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("Could not create canvas");

    let texture_creator = canvas.texture_creator();
    let mut event_pump = ctx.event_pump()?;

    canvas.set_draw_color(Color::RGB(136, 192, 208));
    canvas.clear();
    canvas.present();

    let mut player = CharacterBody {
        texture: texture_creator.load_texture(&Path::new("assets/reaper.png"))?,
        position: Point::new(0, 0),
        sprite: Rect::new(0, 0, 26, 36),
        speed: 10,
    };

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running Ok(()),
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    player.position = player.position.offset(-player.speed, 0);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    player.position = player.position.offset(player.speed, 0);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    player.position = player.position.offset(0, -player.speed);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    player.position = player.position.offset(0, player.speed);
                }
                _ => {}
            }
        }

        // Game Loop
        canvas.set_draw_color(Color::RGB(136, 192, 208));
        canvas.clear();
        render(&mut canvas, &player)?;

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
