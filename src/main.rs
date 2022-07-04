mod cpu;

const SCALE: i32 = 7;

use std::env;
use std::fs::File;
use std::io::Read;
use raylib::prelude::*;
use raylib::consts::KeyboardKey::*;

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
        .size(64 * SCALE, 32 * SCALE)
        .title("Ultra8")
        .build();

        //rl.set_target_fps(500);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        if d.is_key_pressed(KEY_SPACE) {
            //c8.set_key(14, true);
        }
        println!("{}", c8.keypressed);
        c8.cycle();
        d.clear_background(Color::GREEN);
        let mut pressed = false;
        for i in 0..KEYS.len() {
            if d.is_key_down(KEYS[i]) {
                c8.set_key(i as u8, 1);
            } else {
                c8.set_key(i as u8, 0)
            }
        }

        let gfx = c8.get_graphics();
        if c8.draw {
            for y in 0..32 {
                for x in 0..64 {
                    if gfx[(x + (y * 64)) as usize] == 1 {
                        let pix_y = y * SCALE;
                        let pix_x = x * SCALE;
                        d.draw_rectangle(pix_x, pix_y, SCALE, SCALE, Color::BLACK);

                    }
                }
            }
        }

    }
}
