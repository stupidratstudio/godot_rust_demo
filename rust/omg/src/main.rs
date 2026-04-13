mod map_matrix;
mod objects;

use crate::map_matrix::MapMatrix;
use std::time::Instant;

const XSIZE: u8 = 200;
const YSIZE: u8 = 60;

fn clear_console() {
    //print!("\x1B[2J\x1B[1;1H");
    print!("\x1B[1;1H");
}

fn center_console() {
    print!("\x1B[H");
}

fn main() {
    //bench(); return;
    center_console();

    let mut iteration = 0;
    let mut map = MapMatrix::new(XSIZE, YSIZE);
    map.init();

    loop {
        iteration += 1;
        if iteration == 100 {
            map.init();
            iteration = 0;
        }

        clear_console();
        print!("{}", map);
        std::thread::sleep(std::time::Duration::from_millis(10));

        map.update();
    }
}

fn bench() {
    let start = Instant::now();
    let mut map = MapMatrix::new(255, 255);
    let iterations = 1_000;
    map.init();
    for _ in 0..iterations {
        map.update();
    }
    let duration = start.elapsed();
    println!("Benchmark: {:?}", duration.div_f32(iterations as f32));
}
