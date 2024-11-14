
extern crate sdl2;

use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::ttf::Font;
use sdl2::video::Window;
use sdl2::Sdl;
use std::time::Duration;

const WINDOW_WIDTH: u32 = 128;
const WINDOW_HEIGHT: u32 = 64;

pub fn screen() -> Result<(), String> {
    // 初始化SDL2和TTF
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

    // 创建窗口
    let window = video_subsystem
        .window("Hello, World!", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    // 设置背景颜色
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    // 加载字体并渲染文本
    let font_path = "/Library/Fonts/Arial Unicode.ttf"; // 替换为系统中存在的字体路径
    let font = ttf_context.load_font(font_path, 128)?;
    let surface = font
        .render("Hello, World!")
        .blended(Color::RGB(255, 255, 255))
        .map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;
    
    // 将文本绘制到屏幕中央
    let target = Rect::new(100, 200, surface.width(), surface.height());
    canvas.copy(&texture, None, Some(target))?;
    canvas.present();

    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                _ => {}
            }
        }
        // 控制帧率
        std::thread::sleep(Duration::from_millis(100));
    }

    Ok(())
}
