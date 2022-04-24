extern crate sdl2;
extern crate rand;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use crate::rand::Rng;

struct WaveFunction {
    options: Vec<u32>,
    final_value: u32,
    x: i32,
    y: i32
}

struct Board {
    tiles: Vec<WaveFunction>,
    width: usize,
    height: usize
}

impl WaveFunction {
    fn new(x: i32, y: i32, options: &[u32]) -> Self {
        Self { options: options.to_vec(), final_value: 0, x, y }
    }

    fn collapse(&mut self) {
        let mut rng = rand::thread_rng();
        let idx = rng.gen_range(0..self.options.len());
        self.final_value = self.options[idx];
        self.options.clear();
    }

    fn entropy(&self) -> usize {
        self.options.len()
    }

    fn colour(&self) -> Color {
        if self.entropy() == 0 {
            Color::RGB(
                ((self.final_value >> 16) & 0xFF) as u8,
                ((self.final_value >> 8)  & 0xFF) as u8,
                (self.final_value & 0xFF) as u8)
        } else {
            let mut r: u8 = 0;
            let mut g: u8 = 0;
            let mut b: u8 = 0;
            for colour in &self.options {
                r += (((colour >> 16) & 0xFF) / self.entropy() as u32) as u8;
                g += (((colour >> 8)  & 0xFF) / self.entropy() as u32) as u8;
                b +=  ((colour & 0xFF) / self.entropy() as u32) as u8;
            }
            Color::RGB(r, g, b)
        }
    }
}

impl Board {
    fn new(w: usize, h: usize, possible_tiles: &[u32]) -> Self {
        let mut tiles: Vec<WaveFunction> = Vec::with_capacity(w * h);
        for i in 0..(w*h) {
            tiles.push(WaveFunction::new((i % w) as i32, (i / w) as i32, possible_tiles));
        }

        Board {
            tiles,
            width: w, height: h
        }
    }

    fn iterate(&mut self) {
        let min_entropy = self.tiles.iter().min_by_key(|tile| {
            if tile.entropy() > 0 {
                tile.entropy()
            } else {
                usize::MAX
            }
        }).unwrap();
        let min_entropy = min_entropy.entropy();
        if min_entropy == 0 {
            return;
        }
        let mut tiles_with_min = self.tiles.iter_mut().filter(|tile| tile.entropy() == min_entropy).collect::<Vec<&mut WaveFunction>>();
        
        let mut rng = rand::thread_rng();
        let idx = rng.gen_range(0..tiles_with_min.len());
        let tile = &mut tiles_with_min[idx];

        tile.collapse();
        let x = tile.x;
        let y = tile.y;
        self.propogate(x, y);
    }

    fn propogate(&mut self, x: i32, y: i32) {
        
    }
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let winwidth = 800;
    let winheight = 600; 

    let window = video_subsystem.window("rust-sdl2 demo", winwidth, winheight)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;

    let possible_tiles = [0xFF0000, 0x00FF00, 0x0000FF];

    let width = 30;
    let height = 30;
    
    let mut board = Board::new(width, height, &possible_tiles);
    let tw = std::cmp::min(winwidth as usize / board.width, winheight as usize / board.height);
    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        for tile in &board.tiles {
            canvas.set_draw_color(tile.colour());

            let x = tile.x * tw as i32;
            let y = tile.y * tw as i32;
            
            canvas.fill_rect(Rect::new(x, y, tw as u32, tw as u32)).unwrap();
        }
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        board.iterate();

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
