extern crate minifb;

use std::thread;

use minifb::WindowOptions;

struct GameBoy {
    running: bool
}

struct Screen {
    window: minifb::Window,
    buffer: Vec<u32>
}


impl Screen {
    pub fn new(width: usize, height: usize) -> Screen {
        println!("Screen::new");
        let window = match minifb::Window::new("debug", width, height,
                                                   WindowOptions::default()) {
            Ok(win) => win,
            Err(err) => {
                panic!("Unable to create window {}", err);
            }
        };

        Screen {
            window: window,
            buffer: vec![0; width * height]
        }
    }

    pub fn run(&mut self) {
        println!("Screen::run");
        for i in self.buffer.iter_mut() {
            *i = 70; // write something more funny here!
        }
        self.window.update_with_buffer(&self.buffer);
        self.window.set_title("Screen running");
    }
}

impl GameBoy {
    pub fn new(width: usize, height: usize) -> GameBoy {
        println!("GameBoy::new");

        let _ = thread::spawn(move || {
            println!("GameBoy::new in thread");
            let mut screen = Screen::new(width, height);
            screen.run();
        });

        GameBoy {
            running: false
        }
    }

    pub fn run(&mut self) {
        println!("GameBoy::run");
        self.running = true;
    }
}

fn main() {
    let mut gameboy = GameBoy::new(160, 288);
    gameboy.run();
}
