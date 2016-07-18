extern crate minifb;
use minifb::WindowOptions;

use std::thread;
use std::sync::{Arc, RwLock};

use std::thread::sleep;
use std::time::{Duration, Instant};

struct Cpu {
    counter: Arc<RwLock<usize>>,
}

impl Cpu {
    pub fn new(counter: Arc<RwLock<usize>>) -> Cpu {
        Cpu { counter: counter }
    }

    pub fn update(&mut self) {
        let mut counter_reference = self.counter.write().unwrap();
        *counter_reference = *counter_reference + 1;
        println!("Cpu::update counter to {}", *counter_reference);
    }

    pub fn run(&mut self) {
        println!("Cpu::run");
        let cpu_tick_duration = Duration::from_millis(100);
        let mut previous_tick = Instant::now();

        loop {
            let now = Instant::now();
            if now - previous_tick > cpu_tick_duration {
                self.update();
                previous_tick = now;
            };
            sleep(cpu_tick_duration / 100);
        }
    }
}

struct Screen {
    window: minifb::Window,
    buffer: Vec<u32>,
    counter: Arc<RwLock<usize>>,
}

impl Screen {
    pub fn new(counter: Arc<RwLock<usize>>) -> Screen {
        let window = minifb::Window::new("Hello threaded rust 🔥", 640, 480, WindowOptions::default()).unwrap();

        Screen {
            counter: counter,
            buffer: vec![0; window.get_size().0 * window.get_size().1],
            window: window,
        }
    }

    pub fn draw(&mut self) {
        let counter = {
            *self.counter.read().unwrap()
        } as u32;
        println!("Screen::draw read {} from shared counter", counter);

        for pixel in self.buffer.iter_mut() {
            *pixel = counter << 16 | (255 - counter) << 8 | counter;
        }

        self.window.update_with_buffer(&self.buffer);
    }

    pub fn run(&mut self) {
        println!("Screen::run");
        let frame_duration = Duration::from_millis(16);
        let mut previous_draw = Instant::now();

        loop {
            let now = Instant::now();
            if now - previous_draw > frame_duration {
                self.draw();
                previous_draw = now;
            };
            sleep(frame_duration / 100);
        }
    }
}

fn main() {
    let counter = Arc::new(RwLock::new(0));
    let counter_ref = counter.clone();
    let mut screen = Screen::new(counter);

    let _ = thread::spawn(move || {
        let mut cpu = Cpu::new(counter_ref);
        cpu.run();
    });

    screen.run();
}
