const MEMSIZE: usize = 4096;

const PROGRAM_START: usize = 0x200;

pub struct Cpu {
	v: [u8; 16],
	index: u16,

	mem: [u8; MEMSIZE],

	pc: u16,
	sp: u16,

	opcode: u16,

	pub display: [u8; 64 * 32],
	stack: [u16; 16],
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

			display: [0; 64 * 32],
			stack: [0;16],
		}
	}

	pub fn load(&mut self, content: &[u8]) {
		for i in 0..content.len() {
			self.mem[i + PROGRAM_START] = content[i];
		}
	}

	fn fetch(&mut self) {
		let a = self.mem[self.pc as usize];
		let b = self.mem[(self.pc + 1) as usize];

		self.opcode = (((a as u16) << 8) | b as u16) as u16;

		println!("{:#2x}", self.opcode);
	}

	pub fn cycle(&mut self) {
		println!("{:#2x}", self.opcode);
		self.fetch();

		match self.opcode & 0xF000 {
			0x0000 => {
				match self.opcode & 0x00FF {
					0xE0 => self.ins_0x00E0(),
					0xEE => self.ins_0x00EE(),
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
			0xD000 => self.ins_D000(),
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
					_ => {}//unimplemented!("Opcode {:#2x}", self.opcode),
				}
			}

			_ => {}//unimplemented!("Opcode {:#2x}", self.opcode),
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
	// fn stack_pop(&mut self) -> u16 {

		

	// 	self.stack[self.sp as usize]
	// 	self.sp -= 1;
	// }

	fn stack_push(&mut self, value: u16) {
		self.sp += 1;
		self.stack[self.sp as usize] = value;
	}
}

// Chip8 Instructions

impl Cpu {
	fn ins_0x00E0(&mut self) {
		self.display = [0; 64 * 32];
		self.pc += 2;
	}

	fn ins_0x00EE(&mut self) {
		//self.stack[self.sp as usize] = self.pc;
		//self.sp -= 1;
		self.pc = self.stack[self.sp as usize] + 2;

		//self.pc = self.stack[sp] + 2;
	}

	fn ins_0x1000(&mut self) {
		self.pc = self.get_nnn();
	}

	fn ins_0x2000(&mut self) {
		self.stack_push(self.pc);
		self.pc = self.get_nnn();
		//self.pc += 2;
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
			self.pc += 4;
		} else {
			self.pc += 2;
		}
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

	fn ins_0x8006(&mut self) {}

	fn ins_0x8007(&mut self) {}

	fn ins_0x800E(&mut self) {}

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

	fn ins_D000(&mut self) {

		let height = self.get_n();
		let x = self.v[self.get_vx() as usize] % 64;
		let y = self.v[self.get_vy() as usize] % 32;

		self.v[0xF] = 0;
		for yline in 0..height {
			let pixel = self.mem[(self.index + yline) as usize];
			for xline in 0..8 {
				if pixel & (0x80 >> xline) != 0 {
					let a = (x as u16 + xline) as u16;
					let b = (y as u16 + yline) as u16;
					if self.display[(a + (b * 64)) as usize] == 1 {
						self.v[0xF] = 1;
					}
					self.display[(a + (b * 64)) as usize] ^= 1;

					// let a = ((x as u16).wrapping_add(xline)) as u16;
					// let b = ((y as u16).wrapping_add(yline)) as u16;

					// if self.display[a.wrapping_add(b.wrapping_mul(64)) as usize] == 1{

					// }

					// self.display[a.wrapping_add(b.wrapping_mul(64)) as usize] ^= 1;
				}
			}
		}

		self.pc += 2;
	}
}

// Misc functions
impl Cpu {
	pub fn get_graphics(&self) -> [u8; 64 * 32] {
		self.display
	}
}

#[cfg(test)]
mod tests {
	use super::*;
    #[test]
    fn stack_test() {
    	let mut c8 = Cpu::init();

    	c8.stack_push(0x10);

    	assert!(c8.stack_pop() == 0x10);
    }
}