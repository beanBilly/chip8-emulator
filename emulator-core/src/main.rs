use minifb::{Key, Scale, Window, WindowOptions};
//to access files
use std::fs::File;
//allowing compiler to read the files
use std::io::Read;
//allowing compiler to access the chip8 architecture
use emulator_core::chip8;
fn main(){
	  //creating chip emulator here
	  let mut emulator = chip8::new();
	  
	  //given permission for compiler to read the hard drive, we are trying to read the file "pong.ch8) to load into the ROM
	  let mut romFile = File::open("pong.ch8").expect("Failed to open ROM files");
	  //we using vector to load the game files dynamically to avoid wasting memory
	  let mut romData = Vec::new();
	  //read file to ROM
	  romFile.read_to_end(&mut romData).expect("Unable to load data to ROM");
	  
	  //loading ROM data to RAM
	  emulator.loadRom(&romData);
	  
	  //initializing the property of our "OS" or visual application 
	  let mut options = WindowOptions::default();
	  options.scale = Scale::X10;
	  
	  let mut window = Window::new(
    		"CHIP-8 Emulator", // Title
    		64,                // base width
    		32,                // base height
    		options,           // upscale settings, amplifies our base by 10 as we mentioned
    		).unwrap_or_else(|e| { // closure function
    			panic!("Could not open window: {}", e); //when 
			});
			
	//forced the host os to run only 60 frames per second, 166667 ms for each frame
	window.limit_update_rate(Some(std::time::Duration::from_micros(16667)));
	
	//creating the blank pixel buffer
	let mut pixel_buffer = [0u32; 2048];
	
	//game loop, run until window close or esc pressed
	while window.is_open() && !window.is_key_down(Key::Escape) {
		//take opcode from the RAM,decode it, update the timer (for 1 of 60 fps)
		let opcode = emulator.fetch();
		emulator.decode(opcode);
		emulator.updateTimer();
		
		//emulator's boolean screen into  u32 colour buffer
        	for i in 0..2048 {
            		if emulator.display[i] == true {
                		pixel_buffer[i] = 0xFFFFFF; // Bright White pixel
            		} else {
                		pixel_buffer[i] = 0x000000; // Pitch Black pixel
            		}
       		}
       		
       		window
       			.update_with_buffer(&pixel_buffer, 64, 32)
            		.unwrap_or_else(|e| {
                			panic!("Failed to update window buffer: {}", e);
                			});
	}

	  
	  
	  
	  
	
}
