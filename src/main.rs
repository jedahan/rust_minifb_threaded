extern crate minifb;

use std::thread;
use std::time::Duration;
use std::thread::sleep;

use minifb::{Key, WindowOptions};

const WIDTH: usize = 288;
const HEIGHT: usize = 160;

fn main() {
    let mut buffer = vec![0; WIDTH * HEIGHT];

    let _ = thread::spawn(move || {
        let mut window = minifb::Window::new("debug", WIDTH, HEIGHT, WindowOptions::default()).unwrap();
        window.set_title("Screen loop");
        
        while window.is_open() && !window.is_key_down(Key::Escape) {
            for i in buffer.iter_mut() {
                *i = 70; // write something more funny here!
            }
            window.update_with_buffer(&buffer);
            sleep(Duration::from_millis(16));
        }
    });

    println!("hi from the main thread");
}
