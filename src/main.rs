mod cpu;

use std::fs::File;
use std::io::Read;
use raylib::prelude::*;

fn main() {
    println!("Hello, world!");
    let mut file = File::open("test.ch8").expect("File failed to read!");
    let mut buf: Vec<u8> = Vec::new();
    file.read_to_end(&mut buf);

    let mut c8 = cpu::Cpu::init();
    c8.load(&buf);
    

    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Hello, World")
        .build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        c8.cycle();
        d.clear_background(Color::BLACK);


        let gfx = c8.get_graphics();
        for y in 0..32 {
            for x in 0..64 {
                if gfx[(x + (y * 64)) as usize] == 1 {
                    let pix_y = y * 8;
                    let pix_x = x * 8;
                    //println!("X: {}, Y: {}", pix_x, pix_y);
                    d.draw_rectangle(pix_x, pix_y, 8, 8, Color::WHITE);
                }

            }
        }
    }
}
