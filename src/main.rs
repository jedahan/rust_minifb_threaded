extern crate minifb;

use std::thread;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use std::thread::sleep;

const MEMORY_SIZE: usize = 16;

pub struct GameBoy {
    cpu: Cpu
}

pub struct Cpu {
    running: bool,
    memory: Arc<RwLock<[u8; MEMORY_SIZE]>>,
}

pub struct Screen {
    window: minifb::Window,
    memory: Arc<RwLock<[u8; MEMORY_SIZE]>>,
}

impl Cpu {
    pub fn new(memory: Arc<RwLock<[u8; MEMORY_SIZE]>>) -> Cpu {
        println!("Cpu::new");
        Cpu {
            running: false,
            memory: memory,
        }
    }

    pub fn run(&mut self) {
        println!("Cpu::run");
        self.running = true;
        self.memory.write().unwrap()[0] = 7;
        println!("  [0x00] {:?}", self.memory.read().unwrap()[0]);
    }
}

impl Screen {
    pub fn new(width: usize, height: usize, memory: Arc<RwLock<[u8; MEMORY_SIZE]>>) -> Screen {
        println!("Screen::new");
        let window = minifb::Window::new("debug",
                                         width,
                                         height,
                                         minifb::WindowOptions {
                                             borderless: true,
                                             scale: minifb::Scale::X4,
                                             ..Default::default()
                                         })
            .unwrap();

        Screen {
            window: window,
            memory: memory,
        }
    }

    pub fn run(&mut self) {
        let first_byte = self.memory.read().unwrap()[0];
        println!("Screen::run");
        println!("  [0x00] {:?}", first_byte);
        let title = format!("We are running now! {:?}", first_byte);
        self.window.set_title(title.as_str());
    }
}

impl GameBoy {
    pub fn new(width: usize, height: usize) -> GameBoy {
        println!("GameBoy::new");
        let memory = Arc::new(RwLock::new([0; MEMORY_SIZE]));

        let memory_ref = memory.clone();
        println!("GameBoy::new memory_ref cloned");

        let _ = thread::spawn(move || {
            println!("GameBoy::new in thread");
            let mut screen = Screen::new(width, height, memory_ref);
            screen.run();
        });

        sleep(Duration::from_millis(1000));

        GameBoy {
            cpu: Cpu::new(memory)
        }
    }

    pub fn run(&mut self) {
        println!("Gameboy::run");
        self.cpu.run();
    }
}

fn main() {
    let mut gameboy = GameBoy::new(160, 288);
    gameboy.run();
}
