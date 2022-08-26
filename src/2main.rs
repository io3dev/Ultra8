use ggez::conf::{WindowSetup, NumSamples, WindowMode};
use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color};
use ggez::event::{self, EventHandler, KeyCode, KeyMods};
use ggez_egui::{egui, EguiBackend};
use ggez::input::mouse::MouseButton;
mod cpu;
use cpu::Cpu;

// Offset of the game window from the left side of the screen
const X_OFFSET: usize = 100;
const Y_OFFSET: usize = 100;

fn main() {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .window_mode(WindowMode {
            width: 800.0,
            height: 550.0,
            ..Default::default()
        })
        .window_setup(WindowSetup {
            title: "CHIP8 Emulator".to_owned(),
            samples: NumSamples::One,
            vsync: false,
            icon: "".to_owned(),
            srgb: true,
        })
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let my_game = MyGame::new(&mut ctx);

    // Run!
    event::run(ctx, event_loop, my_game);
}

struct MyGame {
    chip8: Cpu,
    egui_backend: EguiBackend,

    chip8_running: bool,
}

use std::env;
use std::fs::File;
use std::io::Read;

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        // Load/create resources such as images here.
        MyGame {
            chip8: {
                let args: Vec<String> = env::args().collect();
                if args.len() == 1 {
                    panic!("No argument specified!");
                }
                let mut file = File::open(&args[1]).expect("File failed to read!");
                let mut buf: Vec<u8> = Vec::new();
                file.read_to_end(&mut buf);
            
                let mut c8 = cpu::Cpu::init();
                c8.load(&buf);

                c8
            },

            egui_backend: EguiBackend::default(),
            chip8_running: true,
        }
    }
}
const SCALE: i32 = 7;
impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        let egui_ctx = self.egui_backend.ctx();
		egui::Window::new("Chip8 Control").show(&egui_ctx, |ui| {
            ui.label(format!("{}", ggez::timer::fps(_ctx) as usize));
            ui.checkbox(&mut self.chip8_running, "Running");
            if ui.button("Cycle").clicked() {
                self.chip8.cycle();
                // todo!();
            }
            if ui.button("Dump Ram").clicked() {
                // println!("{}", self.chip8.ra)
                todo!();
            }
            
		});
        egui::Window::new("CHIP8 Registers").show(&egui_ctx, |ui|{
            ui.label(format!("{:#?}", self.chip8.v));
        }
        );
        if self.chip8_running {
            self.chip8.cycle();
        }
        
        if ggez::input::keyboard::is_key_pressed(_ctx, KeyCode::Q) {
            self.chip8.set_key(4,1);
        } else {
            self.chip8.set_key(4,0);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        
        graphics::clear(ctx, Color::BLACK);
        
        // Draw code here...
        

        // let gfx = c8.get_graphics();
        // if c8.draw {
        //     for y in 0..32 {
        //         for x in 0..64 {
        //             if gfx[(x + (y * 64)) as usize] == 1 {
        //                 let pix_y = y * SCALE;
        //                 let pix_x = x * SCALE;
        //                 d.draw_rectangle(pix_x, pix_y, SCALE, SCALE, Color::RED);

        //             } else {
        //                 let pix_y = y * SCALE;
        //                 let pix_x = x * SCALE;
        //                 d.draw_rectangle(pix_x, pix_y, SCALE, SCALE, Color::BLACK);
        //             }
        //         }
        //     }
        // }

        let framebuffer = self.chip8.get_graphics();
        if self.chip8.draw {
            for y in 0..32 {
                for x in 0..64 {
                    if framebuffer[x + (y * 64) as usize] == 1 {
                        let pix_y = (y * SCALE) + Y_OFFSET as i32;
                        let pix_x = (x * (SCALE as usize)) + X_OFFSET;
                        let pixel = 
                        graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), graphics::Rect::new(pix_x as f32, pix_y as f32, SCALE as f32, SCALE as f32), graphics::Color::WHITE).unwrap();
                        graphics::draw(ctx, &pixel, graphics::DrawParam::default()).expect("ASd");
                        // println!("{}, {}", pix_x, pix_y);
                        
                    }
                }
            }
        }
        graphics::draw(ctx, &self.egui_backend, ([0.0, 0.0],))?;
        graphics::present(ctx).unwrap();
        
        
        Ok(())
    }

	fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, _x: f32, _y: f32) {
		self.egui_backend.input.mouse_button_down_event(button);
	}

	fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, _x: f32, _y: f32) {
		self.egui_backend.input.mouse_button_up_event(button);
	}

	fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
		self.egui_backend.input.mouse_motion_event(x, y);
	}
}