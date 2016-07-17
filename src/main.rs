extern crate minifb;

use std::thread;
use std::time::Duration;
use std::thread::sleep;

use minifb::WindowOptions;

const WIDTH: usize = 288;
const HEIGHT: usize = 160;

fn main() {
    let mut buffer = vec![0; WIDTH * HEIGHT];
    let mut frames: u32 = 0;

    let _ = thread::spawn(move || {
        println!("Hello from child thread");
        let mut window = minifb::Window::new("debug", WIDTH, HEIGHT, WindowOptions::default()).unwrap();
        println!("Hello from after window creation in the child thread");

        while frames < 0xFF {
            println!("Hello from frame {}", frames);
            for i in buffer.iter_mut() {
                *i = frames;
            }
            window.update_with_buffer(&buffer);
            frames = frames + 1;

            sleep(Duration::from_millis(16));
        }
    });

    loop {
        println!("Hi from the main thread");
        sleep(Duration::from_millis(10000));
    }
}
