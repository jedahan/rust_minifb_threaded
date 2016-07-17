extern crate minifb;

use std::thread;
use std::time::Duration;
use std::thread::sleep;

use minifb::{Key, WindowOptions};

const WIDTH: usize = 288;
const HEIGHT: usize = 160;

fn main() {
    let mut buffer = vec![0; WIDTH * HEIGHT];
    let mut frames: usize = 0;

    let handle = thread::spawn(move || {
        let mut window = minifb::Window::new("debug", WIDTH, HEIGHT, WindowOptions::default()).unwrap();

        while window.is_open() && !window.is_key_down(Key::Escape) {
            let color = (frames % 255) as u32;
            for i in buffer.iter_mut() {
                *i = color;
            }

            window.update_with_buffer(&buffer);
            window.set_title(format!("Screen is color {}", color).as_str());
            frames = frames + 1;

            sleep(Duration::from_millis(16));
        }

        "Thread has ended"
    });

    println!("{}", handle.join().unwrap());

    loop {
        println!("Hi from the main thread");
        sleep(Duration::from_millis(10000));
    }
}
