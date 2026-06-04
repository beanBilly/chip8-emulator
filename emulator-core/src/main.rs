use minifb::{Key, Scale, Window, WindowOptions};
//to access files
use std::fs::File;
//allowing compiler to read the files
use std::io::Read;
//allowing compiler to access the chip8 architecture
use rodio::{DeviceSinkBuilder, Player};
//devicesinkbuilder opens the OS sound hardware, and player acts as the cassette player
use rodio::source::{SineWave, Source};
//sinewave is mathematical signal generator, course for filters like amplify() and repeat_infinite()
use std::time::Duration;
//sound timing
use std::panic;
//for debugging
use emulator_core::Chip8;
fn main(){
	panic::set_hook(Box::new(|panic_info| {
        eprintln!("{panic_info}");
        std::process::exit(1);
        }));
	  //creating chip emulator here
	  let mut emulator = Chip8::new();
	  
	  //given permission for compiler to read the hard drive, we are trying to read the file "pong.ch8") to load into the ROM
	  let mut romFile = File::open("pong.ch8").expect("Failed to open ROM files");
	  //we using vector to load the game files dynamically to avoid wasting memory
	  let mut romData = Vec::new();
	  //read file to ROM
	  romFile.read_to_end(&mut romData).expect("Unable to load data to ROM");

/*	  	  
let romData: Vec<u8> = vec![

];
*/
	  //loading ROM data to RAM
	  emulator.loadRom(&romData);
	  
	  //initializing the property of our "OS" or visual application 
	  let mut options = WindowOptions::default();
	  options.scale = Scale::X16;
	  
	  let mut window = Window::new(
    		"CHIP-8 Emulator", // Title
    		64,                // base width
    		32,                // base height
    		options,           // upscale settings, amplifies our base by 10 as we mentioned
    		).unwrap_or_else(|e| { // closure function
    			panic!("Could not open window: {}", e); //when 
			});

	
	//connecting to physical speaker		
	let handle = DeviceSinkBuilder::open_default_sink()
        	.expect("Failed to open default audio stream");
        //connect to sound card
        let player = Player::connect_new(&handle.mixer());
        
        let source = SineWave::new(440.0)
        	.amplify(0.15)
        	.repeat_infinite();
        	
        player.append(source);
        //std::thread::sleep(std::time::Duration::from_millis(50));
        player.pause();
        
	//forced the host os to run only 60 frames per second, 166667 ms for each frame
	window.limit_update_rate(Some(std::time::Duration::from_micros(16667)));
	
	//creating the blank pixel buffer
	let mut pixelBuffer = [0u32; 2048];
	
	//game loop, run until window close or esc pressed
	while window.is_open() && !window.is_key_down(Key::Escape) {
                
                emulator.set_key(0x1, window.is_key_down(Key::Key1));
		emulator.set_key(0x2, window.is_key_down(Key::Key2));
		emulator.set_key(0x3, window.is_key_down(Key::Key3));
		emulator.set_key(0xC, window.is_key_down(Key::Key4));

		emulator.set_key(0x4, window.is_key_down(Key::Q));
		emulator.set_key(0x5, window.is_key_down(Key::W));
		emulator.set_key(0x6, window.is_key_down(Key::E));
		emulator.set_key(0xD, window.is_key_down(Key::R));

		emulator.set_key(0x7, window.is_key_down(Key::A));
		emulator.set_key(0x8, window.is_key_down(Key::S));
		emulator.set_key(0x9, window.is_key_down(Key::D));
		emulator.set_key(0xE, window.is_key_down(Key::F));

		emulator.set_key(0xA, window.is_key_down(Key::Z));
		emulator.set_key(0x0, window.is_key_down(Key::X));
		emulator.set_key(0xB, window.is_key_down(Key::C));
		emulator.set_key(0xF, window.is_key_down(Key::V));
                
                /* js cant access array of bool
		//resetting all key map to false
		emulator.key = [false; 16];

		//check which key is held down when that frame runs
		if window.is_key_down(Key::Key1) { emulator.key[0x1] = true; }
		if window.is_key_down(Key::Key2) { emulator.key[0x2] = true; }
		if window.is_key_down(Key::Key3) { emulator.key[0x3] = true; }
		if window.is_key_down(Key::Key4) { emulator.key[0xC] = true; }

		if window.is_key_down(Key::Q)    { emulator.key[0x4] = true; }
		if window.is_key_down(Key::W)    { emulator.key[0x5] = true; }
		if window.is_key_down(Key::E)    { emulator.key[0x6] = true; }
		if window.is_key_down(Key::R)    { emulator.key[0xD] = true; }

		if window.is_key_down(Key::A)    { emulator.key[0x7] = true; }
		if window.is_key_down(Key::S)    { emulator.key[0x8] = true; }
		if window.is_key_down(Key::D)    { emulator.key[0x9] = true; }
		if window.is_key_down(Key::F)    { emulator.key[0xE] = true; }

		if window.is_key_down(Key::Z)    { emulator.key[0xA] = true; }
		if window.is_key_down(Key::X)    { emulator.key[0x0] = true; }
		if window.is_key_down(Key::C)    { emulator.key[0xB] = true; }
		if window.is_key_down(Key::V)    { emulator.key[0xF] = true; }
		*/
		
		//take opcode from the RAM,decode it, update the timer (for 1 of 60 fps)
		for i in 0..10{
		    let opcode = emulator.fetch();
		    emulator.decode(opcode);
		}
		emulator.updateTimer();
	
		if emulator.soundTimer>0{
			player.play();
		}
		else{
			player.pause();
		}
		
		//emulator's boolean screen into  u32 colour buffer
		let displayData=emulator.getDisplay();
        	for i in 0..2048 {
            		if displayData[i] == true {
                		pixelBuffer[i] = 0xFFFFFF; // Bright White pixel
            		} else {
                		pixelBuffer[i] = 0x000000; // Pitch Black pixel
            		}
       		}
       		
       		window
       			.update_with_buffer(&pixel_buffer, 64, 32)
            		.unwrap_or_else(|e| {
                			panic!("Failed to update window buffer: {}", e);
                			});

	}

	  
	  
	  
	  
	
}
