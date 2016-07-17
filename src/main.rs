extern crate minifb;

use std::thread;
use std::sync::{RwLock, Arc};
use std::time::{Duration, Instant};
use std::thread::sleep;

use minifb::WindowOptions;

const WIDTH: usize = 288;
const HEIGHT: usize = 160;

struct Cpu {
    running: bool,
    frame: Arc<RwLock<usize>>
}

impl Cpu {
    pub fn new(frame: Arc<RwLock<usize>>) -> Cpu {
        Cpu {
            running: false,
            frame: frame
        }
    }

    pub fn update(&mut self) {
        self.running = true;
        let frame = *self.frame.read().unwrap();
        let mut frame_handle = self.frame.write().unwrap();

        *frame_handle = frame + 1;
        println!("CPU updated frame to {}", *frame_handle);
    }

    pub fn run(&mut self) {
        println!("Cpu::run");
        let frame_duration = Duration::from_millis(1000);
        let mut previous_draw = Instant::now();

        loop {
            let now = Instant::now();
            if now - previous_draw > frame_duration {
                self.update();
                previous_draw = now;
            };
            sleep(Duration::from_millis(100));
        }
    }

}

struct Screen {
    window: minifb::Window,
    buffer: Vec<u32>,
    frame: Arc<RwLock<usize>>
}

impl Screen {
    pub fn new(width: usize, height: usize, frame: Arc<RwLock<usize>>) -> Screen {

        Screen {
            frame: frame,
            buffer: vec![0; width * height],
            window: minifb::Window::new("debug", width, height, WindowOptions::default()).unwrap()
        }
    }

    pub fn update(&mut self) {
        let frame = *self.frame.read().unwrap();
        println!("Hello from shared frame {}", frame);
        for i in self.buffer.iter_mut() {
            *i = frame as u32;
        }
        self.window.update_with_buffer(&self.buffer);
    }

    pub fn run(&mut self) {
        println!("DebugScreen::run");
        let frame_duration = Duration::from_millis(16);
        let mut previous_draw = Instant::now();

        loop {
            let now = Instant::now();
            if now - previous_draw > frame_duration {
                self.update();
                previous_draw = now;
            };
            sleep(Duration::from_millis(2));
        }
    }
}

fn main() {
    // what we move into the Cpu, for writing
    let frame = Arc::new(RwLock::new(0));

    let frame_ref = frame.clone();

    let mut screen = Screen::new(WIDTH, HEIGHT, frame);

    let _ = thread::spawn(move || {
        println!("Hello from child (cpu) thread");
        let mut cpu = Cpu::new(frame_ref);
        cpu.run();
    });

    screen.run();
}
