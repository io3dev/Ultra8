use std::thread;
use std::time::Duration;
use rand::Rng;

const MEMSIZE: usize = 4096;

const PROGRAM_START: usize = 0x200;

const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;

const FONTS: [u8; 80] = [
  0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
  0x20, 0x60, 0x20, 0x20, 0x70, // 1
  0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
  0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
  0x90, 0x90, 0xF0, 0x10, 0x10, // 4
  0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
  0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
  0xF0, 0x10, 0x20, 0x40, 0x40, // 7
  0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
  0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
  0xF0, 0x90, 0xF0, 0x90, 0x90, // A
  0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
  0xF0, 0x80, 0x80, 0x80, 0xF0, // C
  0xE0, 0x90, 0x90, 0x90, 0xE0, // D
  0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
  0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

#[derive(Debug)]
enum video_mode {
	SCHIP8,
	CHIP8,
}


pub struct Cpu {
	v: [u8; 16],
	index: u16,

	mem: [u8; MEMSIZE],

	pc: u16,
	sp: u16,

	opcode: u16,

	pub display: [u8; SCREEN_WIDTH * SCREEN_HEIGHT],

	pub keypad: [u8; 16],
	pub keypressed: bool,

	pub draw: bool,

	vmode: video_mode,

	stack: [u16; 16],

	dt: u8,
}

impl Cpu {
	pub fn init() -> Self {
		Cpu {
			v: [0; 16],
			mem: [0; MEMSIZE],

			pc: PROGRAM_START as u16,
			sp: 0,

			opcode: 0,

			index: 0,

			display: [0; SCREEN_WIDTH * SCREEN_HEIGHT],
			vmode: video_mode::CHIP8,
			keypad: [0; 16],
			keypressed: false,
			stack: [0;16],
			draw: false,

			dt: 0,
		}
	}

	pub fn load(&mut self, content: &[u8]) {
		// Load rom into memory
		for i in 0..content.len() {
			self.mem[i + PROGRAM_START] = content[i];
		}

		// Also loads fonts aswell
		for i in 0..FONTS.len() {
			self.mem[i] = FONTS[i];
		}
	}

	fn fetch(&mut self) {
		let a = self.mem[self.pc as usize];
		let b = self.mem[(self.pc + 1) as usize];

		self.opcode = (((a as u16) << 8) | b as u16) as u16;
	}

	pub fn cycle(&mut self) {
		self.fetch();
		println!("OP: {:#2x}", self.opcode);
		match self.opcode & 0xF000 {
			0x0000 => {
				match self.opcode & 0x00FF {
					0xE0 => self.ins_0x00E0(),
					0xEE => self.ins_0x00EE(),
					// S-CHIP Instructions
					0xFD => self.ins_0x00FD(),
					0xFE => self.ins_0x00FE(),
					0xFF => self.ins_0x00FF(),
					_ => unimplemented!("Opcode {:#2x}", self.opcode),
				}
			}

			0x1000 => self.ins_0x1000(),
			0x2000 => self.ins_0x2000(),
			0x3000 => self.ins_0x3000(),
			0x4000 => self.ins_0x4000(),
			0x5000 => self.ins_0x5000(),
			0x6000 => self.ins_0x6000(),
			0x7000 => self.ins_0x7000(),
			0x9000 => self.ins_0x9000(),

			0xA000 => self.ins_0xA000(),
			0xB000 => self.ins_0xB000(),
			0xC000 => {
				let mut rng = rand::thread_rng();
				self.v[self.get_vx() as usize] = rng.gen_range(0..255) & self.get_nn();
				self.pc += 2;
			}
			0xD000 => self.ins_D000(),
			0xE000 => {
				match self.opcode & 0x00FF {
					0x9E => self.ins_0xE9E(),
					0xA1 => self.ins_0xEXA1(),
					_ => unimplemented!("Opcode {:#2x}", self.opcode),
				}
			}
			0xF000 => {
				match self.opcode & 0x00FF {
					0x07 => {
						self.v[self.get_vx() as usize] = self.dt;
						self.pc += 2;
					}
					0x18 => self.pc += 2,
					0x29 => self.ins_0xF029(),
					0x33 => self.ins_F033(),
					0x55 => self.ins_F055(),
					0x65 => self.ins_F065(),
					0x1E => self.ins_0xF01E(),
					0x0A => self.ins_0xF00A(),
					0x15 => {
						self.dt = self.get_vx() as u8;
						self.pc += 2;
					}
					_ => unimplemented!("Opcode {:#2x}", self.opcode),
				}
			}
			0x8000 => {
				match self.opcode & 0x000F {
					0x0 => self.ins_0x8000(),
					0x1 => self.ins_0x8001(),
					0x2 => self.ins_0x8002(),
					0x3 => self.ins_0x8003(),
					0x4 => self.ins_0x8004(),
					0x5 => self.ins_0x8005(),
					0x6 => self.ins_0x8006(),
					0x7 => self.ins_0x8007(),
					0xE => self.ins_0x800E(),
					_ => unimplemented!("Opcode {:#2x}", self.opcode),
				}
			}

			_ => unimplemented!("Opcode {:#2x}", self.opcode),
		}
		if self.dt != 0 {
			self.dt -= 1;
			thread::sleep(Duration::from_millis(15));
		}


	}
}

impl Cpu {
	fn get_nnn(&self) -> u16 {
		self.opcode & 0x0FFF
	}

	fn get_nn(&self) -> u8 {
		(self.opcode & 0x00FF).try_into().unwrap()
	}

	fn get_n(&self) -> u16 {
		self.opcode & 0x000F
	}

	fn get_vx(&self) -> u16 {
		(self.opcode & 0x0F00) >> 8
	}

	fn get_vy(&self) -> u16 {
		(self.opcode & 0x00F0) >> 4
	}
}

// Stack functions
impl Cpu {
	fn stack_pop(&mut self) -> u16 {
		self.sp -= 1;
		self.stack[self.sp as usize]
	}

	fn stack_push(&mut self, value: u16) {
		self.stack[self.sp as usize] = value;
		self.sp += 1;
	}
}

// Chip8 Instructions

impl Cpu {
	fn ins_0x00E0(&mut self) {
		self.display = [0; 64 * 32];
		self.draw = true;
		self.pc += 2;
	}

	fn ins_0x00EE(&mut self) {
		self.pc = self.stack_pop();
		self.pc += 2;
	}



	fn ins_0x1000(&mut self) {
		self.pc = self.get_nnn();
	}

	fn ins_0x2000(&mut self) {
		self.stack_push(self.pc);

		self.pc = self.get_nnn();
	}

	fn ins_0x3000(&mut self) {
		if self.v[self.get_vx() as usize] == self.get_nn() {
			self.pc += 4;
		} else {
			self.pc += 2;
		}
	}

	fn ins_0x4000(&mut self) {
		if self.v[self.get_vx() as usize] != self.get_nn() {
			self.pc += 2;
		}
		self.pc += 2;
	}

	fn ins_0x5000(&mut self) {
		let vx = self.v[self.get_vx() as usize];
		let vy = self.v[self.get_vy() as usize];

		if vx == vy {
			self.pc += 4;
		} else {
			self.pc += 2;
		}
	}

	fn ins_0x6000(&mut self) {
		self.v[self.get_vx() as usize] = self.get_nn();
		self.pc += 2;
	}

	fn ins_0x7000(&mut self) {
		//self.v[self.get_vx() as usize] += self.get_nn();
		self.v[self.get_vx() as usize] = self.v[self.get_vx() as usize].wrapping_add(self.get_nn());
		self.pc += 2;
	}

	fn ins_0x8000(&mut self) {
		self.v[self.get_vx() as usize] = self.v[self.get_vy() as usize];
		self.pc += 2;
	}

	fn ins_0x8001(&mut self) {
		let vy = self.v[self.get_vy() as usize];

		self.v[self.get_vx() as usize] |= vy;

		self.pc += 2;
	}

	fn ins_0x8002(&mut self) {
		let vy = self.v[self.get_vy() as usize];

		self.v[self.get_vx() as usize] &= vy;

		self.pc += 2;
	}

	fn ins_0x8003(&mut self) {
		let vy = self.v[self.get_vy() as usize];

		self.v[self.get_vx() as usize] ^= vy;

		self.pc += 2;
	}

	fn ins_0x8004(&mut self) {
		// We convert the result in 16 bits to detect if it goes past 8 bits, and if it does
		// set the appropriate flag
		let result = (self.v[self.get_vx() as usize]) as u16 + (self.v[self.get_vy() as usize]) as u16;
		if result > 255 {
			self.v[0xF] = 1;
		} else {
			self.v[0xF] = 0;
		}
		self.v[self.get_vx() as usize] = result as u8;
		self.pc += 2;
	}

	fn ins_0x8005(&mut self) {
		//let result = (self.v[self.get_vx() as usize]) as u16 - (self.v[self.get_vy() as usize]) as u16;
		let vy = self.v[self.get_vy() as usize];
		if (self.v[self.get_vx() as usize]) > (self.v[self.get_vy() as usize]) {
			self.v[0xF] = 1;
		} else {
			self.v[0xF] = 0;
		}

		self.v[self.get_vx() as usize] = self.v[self.get_vx() as usize].wrapping_sub(vy);

		self.pc += 2;
	}

	fn ins_0x8006(&mut self) {
		let vx = self.get_vx() as usize;
		let vy = self.get_vy() as usize;

		self.v[0xF] = self.v[vx] & 0x1;
		self.v[vx] >>= 1;
		self.v[vx] = self.v[vx] as u8;
		self.pc += 2;

	}

	fn ins_0x8007(&mut self) {
		let vx = self.get_vx() as usize;
		let vy = self.get_vy() as usize;
		let (result, overflow) = self.v[vy].overflowing_sub(self.v[vx]);
		self.v[vx] = result;
		match overflow {
			true => self.v[0xF] = 0,
			false => self.v[0xF] = 1,
		}
		self.pc += 2;
	}

	fn ins_0x800E(&mut self) {
		//v[0xF] = (v[x] shr 7)
		// let vy = self.v[self.get_vy() as usize];
		// self.v[0xF] = self.v[self.get_vx() as usize] >> 7;
		// self.v[self.get_vx() as usize] = vy << 1;
		// self.pc += 2;
		let vx = self.get_vx() as usize;
		self.v[0xF] = self.v[vx] >> 7;
		self.v[vx] = self.v[vx] << 1;
		self.pc += 2;
	}

	fn ins_0x9000(&mut self) {
		let vx = self.v[self.get_vx() as usize];
		let vy = self.v[self.get_vy() as usize];
		if vx != vy {
			self.pc += 4;
		} else {
			self.pc += 2;
		}

	}

	fn ins_0xA000(&mut self) {
		self.index = self.get_nnn();
		self.pc += 2;
	}

	fn ins_0xB000(&mut self) {
		self.pc = self.get_nnn() + self.v[0] as u16;
	}

	fn ins_D000(&mut self) {
		self.vmode = video_mode::CHIP8;

		let height = self.get_n();
		let x = self.v[self.get_vx() as usize];
		let y = self.v[self.get_vy() as usize];
		//println!("SAD {:?}", self.vmode);
		match &self.vmode {
			video_mode::CHIP8 => {
				// Normal chip 8 rendering
				self.v[0xF] = 0;
				for yline in 0..height {
					let pixel = self.mem[(self.index + yline) as usize];
					for xline in 0..8 {
						if pixel & (0x80 >> xline) != 0 {
							let a = ((x as u16 + xline) as u32) % 64;
							let b = ((y as u16 + yline) as u32) % 32;
							if self.display[(a + (b * 64)) as usize] == 1 {
								self.v[0xF] = 1;
							}
							self.display[(a + (b * 64)) as usize] ^= 1;
						}
					}
				}
				self.draw = true;
			}
			video_mode::SCHIP8 => {
				println!("USING CHPI8");
				// Super chip 8 drawing
			}
		}
		// self.v[0xF] = 0;
		// for yline in 0..height {
		// 	let pixel = self.mem[(self.index + yline) as usize];
		// 	for xline in 0..8 {
		// 		if pixel & (0x80 >> xline) != 0 {
		// 			let a = ((x as u16 + xline) as u32) % 64;
		// 			let b = ((y as u16 + yline) as u32) % 32;
		// 			if self.display[(a + (b * 64)) as usize] == 1 {
		// 				self.v[0xF] = 1;
		// 			}
		// 			self.display[(a + (b * 64)) as usize] ^= 1;

		// 			// let a = ((x as u16).wrapping_add(xline)) as u16;
		// 			// let b = ((y as u16).wrapping_add(yline)) as u16;

		// 			// if self.display[a.wrapping_add(b.wrapping_mul(64)) as usize] == 1{

		// 			// }

		// 			// self.display[a.wrapping_add(b.wrapping_mul(64)) as usize] ^= 1;
		// 		}
		// 	}
		// }
		//self.draw = true;
		self.pc += 2;
	}
	fn ins_0xF029(&mut self) {
		self.index = (self.v[self.get_vx() as usize] * 0x5) as u16;
		self.pc += 2;
	}


	fn ins_F033(&mut self) {
		let vx = self.v[self.get_vx() as usize];
		let vy = self.v[self.get_vy() as usize];
		let i = self.index as usize;

		self.mem[i] = vx / 100;
		self.mem[i + 1] = (vx / 10) % 10;
		self.mem[i + 2] = (vx % 100) % 10;
		self.pc += 2;
	}

	fn ins_F055(&mut self) {
		for i in 0..self.get_vx() + 1 {
			self.mem[(self.index + i) as usize] = self.v[i as usize];
			
		}
		self.pc += 2;
	}

	fn ins_F065(&mut self) {
		for i in 0..self.get_vx() + 1 {
			self.v[i as usize] = self.mem[(self.index + i) as usize];
		}
		self.pc += 2;
	}

	fn ins_0xF01E(&mut self) {
		self.index += self.v[self.get_vx() as usize] as u16;
		self.pc += 2;
	}

	fn ins_0xE9E(&mut self) {
		if self.keypad[(self.v[self.get_vx() as usize]) as usize] != 0 {
			self.pc += 4;
		} else {
			self.pc += 2;
		}
	}

	fn ins_0xEXA1(&mut self) {
		if self.keypad[(self.v[self.get_vx() as usize]) as usize] == 0 {
			self.pc += 4;
		} else {
			self.pc += 2;
		}
	}

	fn ins_0xF00A(&mut self) {
		self.keypressed = false;
		let vx = self.get_vx() as usize;
		for i in 0..16 {
			if self.keypad[i] == 1 {
				self.keypressed = true;
				self.v[vx] = i as u8;
			}
		}

		if self.keypressed {
			self.pc += 2;
		}
	}




}

// Super chip 1.0 instructions

impl Cpu {
	fn ins_0x00FD(&self) {
		std::process::exit(1);
	}

	fn ins_0x00FE(&mut self) {
		self.set_mode(video_mode::CHIP8);
		self.pc += 2;
	}

	fn ins_0x00FF(&mut self) {
		self.set_mode(video_mode::SCHIP8);
		self.pc += 2;
	}
}

// XO-Chip Instructions

// http://johnearnest.github.io/Octo/docs/XO-ChipSpecification.html

impl Cpu {
	fn ins_05XY2(&mut self){

	}
}

// Misc functions
impl Cpu {
	pub fn get_graphics(&self) -> [u8; 64 * 32] {
		self.display
	}

	pub fn load_byte_to_memory(&mut self, v: u8, pos: usize) {
		self.mem[pos] = v;
	}

	fn set_mode(&mut self, v: video_mode) {
		self.vmode = v;
	}
}

// Keyboard instructions
impl Cpu {
	pub fn set_key(&mut self, key: u8, pressed: u8) {
		self.keypad[key as usize] = pressed;

	}
}