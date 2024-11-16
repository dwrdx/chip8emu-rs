
extern crate sdl2;

use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::version::revision_number;
use sdl2::video::Window;
use sdl2::Sdl;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use std::sync::mpsc;
use crate::chip8::Sprite;

const WINDOW_WIDTH: u32 = 800;//128;
const WINDOW_HEIGHT: u32 = 600;//64;

#[allow(dead_code)]
pub struct Screen {
    sdl_context: Sdl,
    canvas: Canvas<Window>,
    rx: mpsc::Receiver<Sprite>,
}


pub trait ScreenTrait {
    fn clear(&mut self);
    fn draw_sprite(&mut self, x: i32, y: i32, sprite: Vec<u8>);
    fn draw_point(&mut self, x: i32, y: i32);
    fn draw(&mut self, text : &str);
    fn render(&mut self);
}

impl ScreenTrait for Screen {
    fn clear(&mut self) {
        // set color to black
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
    }

    fn draw_point(&mut self, x: i32, y: i32) {
        // set color to white
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));

        let (w, h) = self.canvas.output_size().unwrap();
        println!("w: {}, h: {}", w, h);

        let point = Point::new(x, y);
        
        // points.fill_with(|| Point::new(rng.gen_range(0..w as i32), rng.gen_range(0..h as i32)));
        // For performance, it's probably better to draw a whole bunch of points at once
        self.canvas.draw_point(point).unwrap();
        // self.canvas.draw_points(points.as_slice()).unwrap();

        self.canvas.present();
    }

    fn draw_sprite(&mut self, x: i32, y: i32, sprite: Vec<u8>) {
        for i in 0..sprite.len() {
            let row = sprite[i];
            for j in 0..8 {
                if row & (0x80 >> j) != 0 {
                    self.draw_point(x + j as i32, y + i as i32);
                }
            }

        }
    }

    fn draw(&mut self, text : &str) {
        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();

        // 加载字体并渲染文本
        let font_path = "/Library/Fonts/Arial Unicode.ttf"; // 替换为系统中存在的字体路径
        let font = ttf_context.load_font(font_path, 128).unwrap();
        let surface = font
            .render(text)
            .blended(Color::RGB(255, 255, 255))
            .map_err(|e| e.to_string()).unwrap();

        let texture_creator = self.canvas.texture_creator();
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string()).unwrap();

        let target = Rect::new(100, 200, surface.width(), surface.height());
        self.canvas.copy(&texture, None, Some(target)).unwrap();
        self.canvas.present();
    }

    fn render(&mut self) {
        let mut event_pump = self.sdl_context.event_pump().unwrap();

        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => {
                        break 'running
                    },
                        _ => {}
                }
            }

            let received = self.rx.recv().unwrap();
            if received.x == 0 && received.y == 0  && received.data.len() == 0 {
                self.clear();
            } else {
                // draw sprite
                self.draw_sprite(received.x, received.y, received.data);
                panic!("sdfsaf");
            }
            // self.draw_point(400, 300);
            std::thread::sleep(Duration::from_millis(100));
        }
    }

}

impl Screen {
    pub fn new(name: &str, receiver: mpsc::Receiver<Sprite>) -> Screen {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window(name, WINDOW_WIDTH, WINDOW_HEIGHT)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string()).unwrap();

        let canvas = window.into_canvas().build().map_err(|e| e.to_string()).unwrap();

        Screen {
            sdl_context: sdl_context, 
            canvas: canvas,
            rx: receiver,
        }
    }

    pub fn window(&self) -> &Window {
        self.canvas.window()
    }

}

