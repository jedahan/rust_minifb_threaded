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

    thread::spawn(move || {
        let mut window = minifb::Window::new("debug", WIDTH, HEIGHT, WindowOptions::default()).unwrap();

        while window.is_open() && !window.is_key_down(Key::Escape) {

            for i in buffer.iter_mut() {
                *i = 70; // write something more funny here!
            }

            window.update_with_buffer(&buffer);
            sleep(Duration::from_millis(16));
            frames = frames + 1;
            let framecount = format!("Hello from screen {} (frames/60)", frames/60);
            window.set_title(framecount.as_str());
        }
    });

    loop {
        println!("hi from the main thread");
        sleep(Duration::from_millis(16000));
    }
}
