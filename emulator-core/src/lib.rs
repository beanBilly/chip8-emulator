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
