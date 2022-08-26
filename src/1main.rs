mod cpu;


use std::env;
use std::fs::File;
use std::io::Read;
use std::thread::sleep;
use std::time::{Duration, Instant};
use raylib::prelude::*;
use raylib::consts::KeyboardKey::*;


const SCALE: i32 = 7;
const KEYS: [KeyboardKey; 16] = [
    KEY_X,
    KEY_ONE,
    KEY_TWO,
    KEY_THREE,
    KEY_Q,
    KEY_W,
    KEY_E,
    KEY_A,
    KEY_S,
    KEY_D,
    KEY_Z,
    KEY_C,
    KEY_FOUR,
    KEY_R,
    KEY_F,
    KEY_V,
];

fn main() {
    let sleep_duration = Duration::from_millis(1);
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        panic!("No argument specified!");
    }
    let mut file = File::open(&args[1]).expect("File failed to read!");
    let mut buf: Vec<u8> = Vec::new();
    file.read_to_end(&mut buf);

    let mut c8 = cpu::Cpu::init();
    c8.load(&buf);
    //c8.load_byte_to_memory(5, 0x1FF);
    //c8.load_byte_to_memory(1, 0x1FE);
    

    let (mut rl, thread) = raylib::init()
        .size(64 * SCALE, 32 * SCALE + 30)
        .title("Ultra8")
        .build();

    let mut updateonce = 0;
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.draw_text("MACHINE: CHIP8", 20, 32 * SCALE + 10,20,Color::RED);
        d.clear_background(Color::BLACK);
        c8.cycle();
        for i in 0..KEYS.len() {
            if d.is_key_down(KEYS[i]) {
                c8.set_key(i as u8, 1);
            } else {
                c8.set_key(i as u8, 0)
            }
        }

        let now = Instant::now();
        //let delta = = now - 
        if updateonce >= 16 {
            updateonce = 0;
            if c8.dt != 0 {
                c8.dt -= 1;
            }
        }
        updateonce += 1;
        println!("ASDASD: {}", updateonce);

        let gfx = c8.get_graphics();
        if c8.draw {
            for y in 0..32 {
                for x in 0..64 {
                    if gfx[(x + (y * 64)) as usize] == 1 {
                        let pix_y = y * SCALE;
                        let pix_x = x * SCALE;
                        d.draw_rectangle(pix_x, pix_y, SCALE, SCALE, Color::RED);

                    } else {
                        let pix_y = y * SCALE;
                        let pix_x = x * SCALE;
                        d.draw_rectangle(pix_x, pix_y, SCALE, SCALE, Color::BLACK);
                    }
                }
            }
        }
        sleep(Duration::new(0, 1_000_000 as u32))

    }
}

// mod cb_parser;
// use std::fs::File;
// use std::io::Read;
// fn main() {
//     let mut p = cb_parser::Parser::init();
//     let mut f = File::open("i.c8b").unwrap();
//     let mut buf = Vec::new();
//     f.read_to_end(&mut buf).unwrap();
//     p.open(&buf);
//     println!("{:#?}", p.prog_start);
// }
