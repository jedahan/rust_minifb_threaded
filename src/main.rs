// This is my first time writing multithreaded code.
//
// It is a minimal example I was creating to figure out why my gameboy emulator[1]
// was showing nothing when I started to go ham on multithreading.
//
// Any suggestions are greatly appreciated.

extern crate minifb;
use minifb::WindowOptions;

// minifb stands for mini framebuffer.
// Its a library created by emoon that makes it
// dirt simple to create a window, draw pixels,
// and interact with the keyboard and mouse on
// OSX, Linux, and Windows.

use std::thread;
use std::sync::{Arc, RwLock};

// For the gameboy emulator I was making, I wanted 2 threads:
//
//   A Cpu thread that could write to some shared memory,
//   and a Debug Screen thread that could read from that shared memory to draw an image of it
//
// We import thread to spawn a child thread, and introduce two interesting structs:
//
//   Arc: automatic reference count.
//
//     An Arc is a smart pointer that knows how many scopes have a reference to it
//     A RwLock, which makes sure we only have one mutable reference and that any
//     immutable access to shared memory between threads are not being mutated while being read
//
//  We will wrap the shared memory in an Arc<RwLock<>>,
//  for reasons that may become clearer later in the code

use std::thread::sleep;
use std::time::{Duration, Instant};

// We import sleep and Duration and Instant so we are not going full speed on the real cpu.
//
// The general idea is to 'tick' the Cpu thread once every second, incrementing the counter,
// and to 'tick' the Debug screen thread at a buttery smooth 60 times per second.
//

const WIDTH: usize = 288;
const HEIGHT: usize = 160;

// The screen needs a height and width, dontchaknow?

struct Cpu {
    counter: Arc<RwLock<usize>>,
}

// The only thing this virtual Cpu needs is access to the shared memory

impl Cpu {
    pub fn new(counter: Arc<RwLock<usize>>) -> Cpu {
        Cpu {
            counter: counter,
        }
    }

    // Cpu::update is where the magic happens
    // I split up the
    pub fn update(&mut self) {
        let mut counter_reference = self.counter.write().unwrap();
        // self.counter is a cloned Arc<RwLock<usize>>
        //
        // First we use the RwLock::write() method to access self.counter
        // as an immutable reference safely.
        //
        // I am kinda thinking of this as now working with an Arc<usize>,
        // which has been a bit confusing, as I would expect to first
        // have to unwrap() the Arc<> part...

        *counter_reference = *counter_reference + 1;

        // ... but that unwrapping happens here. Arc is a smart
        // pointer that is dereferenced nicely with just a *
        //
        // Or maybe I am totally wrong because if I try
        //
        // ```
        //   let mut counter_reference = *self.counter.write().unwrap();
        //   counter_reference = counter_reference + 1;
        // ```
        //
        // The counter_reference never updates...

        println!("CPU updated counter to {}", *counter_reference);
    }

    // For run(), we will sleep for 1 millisecond at a time and check if 100
    // milliseconds have passed. I don't know much about timers, so I guess this
    // means we can be up to 1 ms out of sync when we run Cpu::update
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
            sleep(Duration::from_millis(1));
        }
    }
}

struct Screen {
    window: minifb::Window,
    buffer: Vec<u32>,
    counter: Arc<RwLock<usize>>,
}

impl Screen {
    pub fn new(width: usize, height: usize, counter: Arc<RwLock<usize>>) -> Screen {

        Screen {
            counter: counter,
            buffer: vec![0; width * height],
            window: minifb::Window::new("debug", width, height, WindowOptions::default()).unwrap(),
        }
    }

    pub fn draw(&mut self) {
        let counter = {*self.counter.read().unwrap()} as u32;
        // This is a bit complicated - the reason we wrap the RwLock
        // in {} is so that we stop holding the lock immediately after this read

        // That means that the cpu can update the reference, and all the rest
        // of the code in this function *may* actually be 'out of date'!
        println!("Screen::draw read {} from shared counter", counter);

        // At this point, we should have dropped the read lock since the counter is out of scope!

        // update the buffer with some interesting color based on that read memory...
        for pixel in self.buffer.iter_mut() {
            *pixel = counter << 16 | (255-counter) << 8 | counter;
        }

        self.window.update_with_buffer(&self.buffer);
        // and then tell the minifb window to draw the contents
    }

    // similar to Cpu::run, we sleep for one millisecond and check if
    // 16 milliseconds (~60 frames / second) have passed.

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
            sleep(Duration::from_millis(1));
        }
    }
}

fn main() {
    let counter = Arc::new(RwLock::new(0));
    // counter is our bit of shared memory. Since it is small, maybe using channels
    // would make more sense, but in the emulator I have a semi-complicated struct
    // and would rather reference/dereference until I understand how channels work
    // and if they are safer/just as fast.
    //

    let counter_ref = counter.clone();
    // This is the magic of Arc<> - it allows us to move the cloned, wrapped
    // reference into the cpu thread later, safely.
    //
    // I think safely means in this case, we will not drop the counter
    // until the clones get dropped, but I am not sure.
    //
    // The word drop means something very specific in rust - when a structs lifetime is
    // over, it gets dropped. This is remotely similar to free() in C.
    //

    let mut screen = Screen::new(WIDTH, HEIGHT, counter);

    // At first, it made more sense to have the Cpu be in the main thread, but
    // OSX is very picky about doing graphics stuff in a child thread [3]
    //
    // So our main thread will be a screen, and we pass it along the first counter reference.
    //

    let _ = thread::spawn(move || {
        // thread::spawn is how you make a thread, which we pass in a function
        // move tells the function to automatically move ownership of any referenced variables
        // to inside the thread. In this case, it should just move counter_ref
        //
        // This is still a bit hazy for me, but it seems to work well.
        //
        println!("Hello from child (cpu) thread");
        let mut cpu = Cpu::new(counter_ref);
        // we pass along our cloned Arc<> counter_ref into the cpu, which will update it later
        cpu.run();
    });

    screen.run();
    // At this point, we have both the screen and cpu
    // running in parallel and safely accessing the counter!
}

// [1]: http://github.com/jedahan/rustboy
//      I am trying my best to document the process of making my first emulator

// [3]: I spent a few days of time (over a few weeks) to make this minimal example and track this down.
//      If you are interested, theres some talk about it here https://github.com/emoon/rust_minifb/issues/21
//
