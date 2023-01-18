use crate::memory::framebuffer::FrameBuffer;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::Sdl;

pub struct Video {
    canvas: Canvas<Window>,
    sdl_context: Sdl,
    scale_factor: usize,
}

impl Video {
    pub fn new(scale_factor: usize) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window(
                "chipeite",
                64 * scale_factor as u32,
                32 * scale_factor as u32,
            )
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        Self {
            canvas,
            sdl_context,
            scale_factor,
        }
    }

    pub fn draw(&mut self, fb: &FrameBuffer) -> Result<(), String> {
        self.canvas.clear();
        self.canvas.present();
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        let rect: Vec<Rect> = fb
            .iter()
            .filter_map(|pixel| {
                if !pixel.state {
                    None
                } else {
                    Some(Rect::new(
                        (pixel.x * self.scale_factor) as i32,
                        (pixel.y * self.scale_factor) as i32,
                        self.scale_factor as u32,
                        self.scale_factor as u32,
                    ))
                }
            })
            .collect();
        self.canvas.fill_rects(&rect)?;
        self.canvas.present();
        Ok(())
    }

    pub fn wait_for_key(&self) -> Option<Keycode> {
        let mut event_pump = self.sdl_context.event_pump().unwrap();
        for event in event_pump.wait_iter() {
            match event {
                Event::Quit { .. } => std::process::exit(0),
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    if keycode as i32 >= Keycode::Num0 as i32
                        && keycode as i32 <= Keycode::Num9 as i32
                    {
                        return Some(keycode);
                    }
                    if keycode as i32 >= Keycode::A as i32 && keycode as i32 <= Keycode::F as i32 {
                        return Some(keycode);
                    }
                    continue;
                }
                _ => continue,
            }
        }
        None
    }

    pub fn beep(&self) {
        todo!()
    }
}
