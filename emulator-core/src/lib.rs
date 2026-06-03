const fontSetSize: usize = 80;

const fontSet: [u8;fontSetSize] = [
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

pub struct Chip8{
	pub ram: [u8;4096],
	pub generalPurposeRegister: [u8;16],
	pub addressIndexRegister: u16,
	pub programCounter: u16,
	pub stack: [u16;16],
	pub stackPoint: u8,
	pub delayTimer: u8,
	pub soundTimer: u8,
	pub display: [bool;2048], // 64x32 screen resolution
	pub key: [bool;16]	
}

impl Chip8{
	pub fn new()->Self{
		let mut newRam= [0;4096];
		
		for i in 0..fontSetSize{
			newRam[i]=fontSet[i];
		}
		
		Self{
			ram: newRam,
			generalPurposeRegister: [0;16],
			addressIndexRegister: 0,
			programCounter: 0x200,
			stack: [0;16],
			stackPoint: 0,
			delayTimer: 0,
			soundTimer: 0,
			display: [false;2048],
			key: [false;16]	
		}
	}
	
	pub fn loadRom(&mut self, romData: &[u8]){
		for i in 0..romData.len(){
			self.ram[0x200+i]=romData[i];
		}
	}
	
	pub fn fetch(&mut self)->u16{
		self.programCounter &= 0x0FFF;  // ensure PC stays in 12-bit space
		let lhsU8=self.ram[self.programCounter as usize];
		let rhsU8=self.ram[(self.programCounter+1) as usize];
		let mut opcodeU16: u16;
		opcodeU16=lhsU8 as u16;
		opcodeU16<<=8;
		opcodeU16|=rhsU8 as u16;
		self.programCounter+=2;
		opcodeU16
	}
	
	pub fn decode(&mut self,opcodeU16:u16){
		//op x y n
		let op=(opcodeU16>>12) & 0xF;
		let x=(opcodeU16>>8) & 0xF;
		let y=(opcodeU16>>4) & 0xF;
		let n=opcodeU16 & 0xF;
		let kk=opcodeU16 as u8;
		let nnn=opcodeU16 & 0xFFF;
		
		match op{
			0x0=>{
				//clear display			
				if kk==0xE0{
					self.display=[false;2048];
				}	
				//return from subroutine
				else if kk==0xEE{
				    if self.stackPoint > 0 {
					self.stackPoint -= 1;
					self.programCounter = self.stack[self.stackPoint as usize];
				    }
				}
			}
			0x1=>{
				//jump to nnn
				self.programCounter=nnn;
			}

			0x2=>{
				//call subroutine
				    if (self.stackPoint as usize) < 16 {
					self.stack[self.stackPoint as usize] = self.programCounter;
					self.stackPoint += 1;
				    }
				    self.programCounter = nnn & 0xFFF;
								
			}
			
			0x3=>{
				//skip instruction if register has the same value
				if self.generalPurposeRegister[x as usize]==kk{
					self.programCounter+=2;
				}
			}
			
			0x4=>{
				//skip instruction if register has different value
				if self.generalPurposeRegister[x as usize]!=kk{
					self.programCounter+=2;
				}
			}
			
			0x5=>{
				//skip if both registers are equal
				if self.generalPurposeRegister[x as usize]==self.generalPurposeRegister[y as usize]{
					self.programCounter+=2;
				}
			}
			
			0x6=>{
				//load byte into register
				self.generalPurposeRegister[x as usize]=kk;
			}
			
			0x7=>{
				//add byte to register (using wrapping arithmetic to avoid panic overflows)
				self.generalPurposeRegister[x as usize]=self.generalPurposeRegister[x as usize].wrapping_add(kk);
			}
			
			0x8=>{//arithmetic operations
				if n==0x0{
					//assign y to x
					self.generalPurposeRegister[x as usize]=self.generalPurposeRegister[y as usize];
				}	
				else if n==0x1{
					//bitwise OR
					self.generalPurposeRegister[x as usize]|=self.generalPurposeRegister[y as usize];
				}
				else if n==0x2{
					//bitwise AND
					self.generalPurposeRegister[x as usize]&=self.generalPurposeRegister[y as usize];
				}
				else if n==0x3{
					//bitwise XOR
					self.generalPurposeRegister[x as usize]^=self.generalPurposeRegister[y as usize];
				}
				else if n==0x4{
					//add w carry; result > 255, set vf 
					let (result, overflowed) = self.generalPurposeRegister[x as usize].overflowing_add(self.generalPurposeRegister[y as usize]);
					self.generalPurposeRegister[x as usize] = result;
					self.generalPurposeRegister[0xF] = if overflowed { 1 } else { 0 };
				}
				else if n==0x5{
					//subtract; x>y, set vf
					let (result, overflowed) = self.generalPurposeRegister[x as usize].overflowing_sub(self.generalPurposeRegister[y as usize]);
					self.generalPurposeRegister[x as usize] = result;
					self.generalPurposeRegister[0xF] = if overflowed { 0 } else { 1 };
				}
				else if n==0x6{
					//save lowest bit to vf
                    self.generalPurposeRegister[x as usize] = self.generalPurposeRegister[y as usize];
					self.generalPurposeRegister[0xF] = self.generalPurposeRegister[x as usize] & 0x1;
					self.generalPurposeRegister[x as usize] >>= 1;
				}
				else if n==0x7{
					//subtract reverse; y>x, set vf
					let (result, overflowed) = self.generalPurposeRegister[y as usize].overflowing_sub(self.generalPurposeRegister[x as usize]);
					self.generalPurposeRegister[x as usize] = result;
					self.generalPurposeRegister[0xF] = if overflowed { 0 } else { 1 };
				}
				else if n==0xE{
					//save highest bit to vf
					self.generalPurposeRegister[0xF] = (self.generalPurposeRegister[x as usize] >> 7) & 0x1;
					self.generalPurposeRegister[x as usize] <<= 1;
				}
			}
			
			0x9=>{
				//skip if both registers are differnt
				if self.generalPurposeRegister[x as usize]!=self.generalPurposeRegister[y as usize]{
					self.programCounter+=2;
				}
			}
			
			0xA=>{
				//load address index register
				self.addressIndexRegister = nnn & 0xFFF; 
			}

			0xB=>{
				//jump to location v0+nnn
				self.programCounter=(self.generalPurposeRegister[0] as u16)+nnn;
			}
			
			0xC=>{
				//generate random 0-255, AND w kk, store x
				use rand::prelude::*;
				let mut rng = rand::rng();
				let random: u8 = rng.random::<u8>();
				self.generalPurposeRegister[x as usize]=kk&random;
			}
			
			0xD=>{
				// Cast to usize early to avoid type casting inside the math loops
				let coordinateX = (self.generalPurposeRegister[x as usize] % 64) as usize;
				let coordinateY = (self.generalPurposeRegister[y as usize] % 32) as usize;
				
				self.generalPurposeRegister[0xF] = 0;
				
				for row in 0..n{
					let spriteByte = self.ram[(self.addressIndexRegister+row) as usize];
					
					for col in 0..8{
						let spriteBit = (spriteByte >> (7-col)) & 1;
						
						if spriteBit == 1 {
							let screenX = coordinateX + col;
							let screenY = coordinateY + (row as usize);
							
							// Check individual active pixel positions to prevent array boundary panics
							if screenX < 64 && screenY < 32 {
								let index = screenX + (screenY * 64);
								
								if self.display[index] == true {
									self.generalPurposeRegister[0xF] = 1;
								}
								self.display[index] ^= true;
							}
						}
					} 
				}
				println!("DRAW: I={:04X}, X={}, Y={}, n={}",
				    self.addressIndexRegister,
				    self.generalPurposeRegister[x as usize],
				    self.generalPurposeRegister[y as usize],
				    n
				);
			}

			0xE=>{
				if kk==0x9E{
					//skip if key in vx is pressed
					if self.key[self.generalPurposeRegister[x as usize] as usize]{
						self.programCounter+=2;
					}
				}
				else if kk==0xA1{
					//skip if key in vx is not pressed
					if !self.key[self.generalPurposeRegister[x as usize] as usize]{
						self.programCounter+=2;
					}
				}
			}

			0xF=>{
				if kk==0x07{
					//set vx to delay timer value
					self.generalPurposeRegister[x as usize]=self.delayTimer;
				}
				else if kk==0x0A{
					//wait for key press, store index in vx
					let mut key_pressed = false;
					for i in 0..16{
						if self.key[i]{
							self.generalPurposeRegister[x as usize]=i as u8;
							key_pressed=true;
							break;
						}
					}
					// If no key is down, cycle PC backwards to block execution flow seamlessly
					if !key_pressed{
						self.programCounter-=2;
					}
				}
				else if kk==0x15{
					//set delay timer to vx
					self.delayTimer=self.generalPurposeRegister[x as usize];
				}
				else if kk==0x18{
					//set sound timer to vx
					self.soundTimer=self.generalPurposeRegister[x as usize];
				}
				else if kk==0x1E{
					//add vx to address index register
					self.addressIndexRegister = (self.addressIndexRegister.wrapping_add(self.generalPurposeRegister[x as usize] as u16)) & 0xFFF;
				}
				else if kk==0x29{
					//set address index register to font sprite location for vx
					self.addressIndexRegister = ((self.generalPurposeRegister[x as usize] as u16) * 5) & 0xFFF;
				}
				else if kk==0x33{
					//store bcd representation of vx in memory
					let value = self.generalPurposeRegister[x as usize];
					self.ram[self.addressIndexRegister as usize] = value / 100;
					self.ram[(self.addressIndexRegister+1) as usize] = (value / 10) % 10;
					self.ram[(self.addressIndexRegister+2) as usize] = value % 10;
				}
				else if kk==0x55{
					//store registers v0 through vx in memory
					for i in 0..=(x as usize){
						self.ram[self.addressIndexRegister as usize + i]=self.generalPurposeRegister[i];
					}
					self.addressIndexRegister += (x + 1) as u16;
				}
				else if kk==0x65{
					//load registers v0 through vx from memory
					for i in 0..=(x as usize){
						self.generalPurposeRegister[i]=self.ram[self.addressIndexRegister as usize + i];
					}
					self.addressIndexRegister += (x + 1) as u16;
				}
			}
			_ => {} // Fallback for unimplemented outer opcodes
		}
	}
	
	pub fn updateTimer(&mut self) {
    		if self.delayTimer > 0 { self.delayTimer -= 1; }
    		if self.soundTimer > 0 { self.soundTimer -= 1; }
	}
}
