pub struct chip8{
	pub ram: [u8;4096],
	pub generalPurposeRegister: [u8;16],
	pub addressIndexRegister: u16,
	pub programCounter: u16,
	pub stack: [u16;16],
	pub stackPoint: u8,
	pub delayTimer: u8,
	pub soundTimer: u8,
	pub display: [u8;256],
	pub key: [bool;16]	
}

impl chip8{
	pub fn new()->Self{
		Self{
			ram: [0;4096],
			generalPurposeRegister: [0;16],
			addressIndexRegister: 0,
			programCounter: 0x200,
			stack: [0;16],
			stackPoint: 0,
			delayTimer: 0,
			soundTimer: 0,
			display: [0;256],
			key: [false;16]	
		}
	}
}
