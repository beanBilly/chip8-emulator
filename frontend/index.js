import init, { Chip8 } from '../emulator-core/pkg/emulator_core.js';

const canvas = document.getElementById('chip8-canvas');
const ctx = canvas.getContext('2d');
const SCALE = 16;

const KEY_MAP = {
    '1': 0x1, '2': 0x2, '3': 0x3, '4': 0xC,
    'q': 0x4, 'w': 0x5, 'e': 0x6, 'r': 0xD,
    'a': 0x7, 's': 0x8, 'd': 0x9, 'f': 0xE,
    'z': 0xA, 'x': 0x0, 'c': 0xB, 'v': 0xF
};

const GAME_CONTROLS_INFO = {
    pong: `
        <strong>Pong Controls</strong>
        <div class="control-row btn-w" data-key="w"><span>Up</span><span class="key-badge">W</span></div>
        <div class="control-row btn-s" data-key="s"><span>Down</span><span class="key-badge">S</span></div>
        <div class="control-row btn-a" data-key="a"><span>Left</span><span class="key-badge">A</span></div>
        <div class="control-row btn-d" data-key="d"><span>Right</span><span class="key-badge">D</span></div>
        <div class="control-row btn-e" data-key="e"><span>Start / Restart</span><span class="key-badge">E</span></div>
    `,
    ghostEscape: `
        <strong>Ghost Escape</strong>
        <div class="control-row btn-e" data-key="x"><span>Deploy / Action</span><span class="key-badge">X</span></div>
    `
};

async function loadRomFile(romName) {
    try {
        const response = await fetch(`./${romName}.ch8`);
        if (!response.ok) {
            throw new Error(`Failed to locate ${romName}.ch8 file (Status: ${response.status})`);
        }
        const buffer = await response.arrayBuffer();
        return new Uint8Array(buffer);
    } catch (error) {
        console.error("Error loading the ROM file:", error);
        return null;
    }
}

async function run() {
    await init();
    
    let emulator = new Chip8();
    const gameSelect = document.getElementById('game-select');
    const colourSelect = document.getElementById('color-select');    
    const instructionsBox = document.getElementById('game-instructions');
    
    let currentRomName = gameSelect.value;
    if (instructionsBox) {
        instructionsBox.innerHTML = GAME_CONTROLS_INFO[currentRomName] || 'Use standard layout mapping configuration';
    }
    
    let romData = await loadRomFile(currentRomName);
    if (romData) {
        emulator.loadRom(romData);
        console.log(`successfully fetched and loaded: ${currentRomName}.ch8`);
    }

    gameSelect.addEventListener('change', async (e) => {
        currentRomName = e.target.value;
        
        if (instructionsBox) {
            instructionsBox.innerHTML = GAME_CONTROLS_INFO[currentRomName] || 'Use standard layout mapping configuration';
        }
        
        const newRomData = await loadRomFile(currentRomName);
        if (newRomData) {
            emulator = new Chip8(); 
            emulator.loadRom(newRomData);
            console.log(`swapped emulator profile to: ${currentRomName}.ch8`);
        }
    });

    window.addEventListener('keydown', (e) => {
        const chip8Key = KEY_MAP[e.key.toLowerCase()];
        if (chip8Key !== undefined) {
            emulator.setKey(chip8Key, true);
        }
    });

    window.addEventListener('keyup', (e) => {
        const chip8Key = KEY_MAP[e.key.toLowerCase()];
        if (chip8Key !== undefined) {
            emulator.setKey(chip8Key, false);
        }
    });

    if (instructionsBox) {
        ['touchstart', 'mousedown'].forEach(evtType => {
            instructionsBox.addEventListener(evtType, (e) => {
                const row = e.target.closest('.control-row');
                if (!row) return;
                
                if (e.cancelable) e.preventDefault();
                
                const keyStr = row.getAttribute('data-key');
                const chip8Key = KEY_MAP[keyStr];
                if (chip8Key !== undefined) {
                    emulator.setKey(chip8Key, true);
                }
            });
        });

        ['touchend', 'mouseup', 'mouseleave'].forEach(evtType => {
            instructionsBox.addEventListener(evtType, (e) => {
                const row = e.target.closest('.control-row');
                if (!row) return;
                
                if (e.cancelable) e.preventDefault();
                
                const keyStr = row.getAttribute('data-key');
                const chip8Key = KEY_MAP[keyStr];
                if (chip8Key !== undefined) {
                    emulator.setKey(chip8Key, false);
                }
            });
        });
    }

    function gameLoop() {
        for (let i = 0; i < 10; i++) {
            let opcode = emulator.fetch();
            emulator.decode(opcode);
        }
        emulator.updateTimer();

        ctx.fillStyle = '#010409'; 
        ctx.fillRect(0, 0, canvas.width, canvas.height);
        
        ctx.fillStyle = colourSelect ? colourSelect.value : '#f0883e';
        
        const displayData = emulator.getDisplay();

        for (let i = 0; i < 2048; i++) {
            if (displayData[i] === 1) {
                const x = i % 64;
                const y = Math.floor(i / 64);
                ctx.fillRect(x * SCALE, y * SCALE, SCALE, SCALE);
            }
        }

        requestAnimationFrame(gameLoop);
    }

    requestAnimationFrame(gameLoop);
}

run();


/*
import init, { Chip8 } from '../emulator-core/pkg/emulator_core.js';

const canvas = document.getElementById('chip8-canvas');
const ctx = canvas.getContext('2d');
const SCALE = 16; // Scales 64x32 to 1024x512

const KEY_MAP = {
    '1': 0x1, '2': 0x2, '3': 0x3, '4': 0xC,
    'q': 0x4, 'w': 0x5, 'e': 0x6, 'r': 0xD,
    'a': 0x7, 's': 0x8, 'd': 0x9, 'f': 0xE,
    'z': 0xA, 'x': 0x0, 'c': 0xB, 'v': 0xF
};

//panel structured with interactive HTML button 
const GAME_CONTROLS_INFO = {
    pong: `
        <strong>Pong Controls</strong>
        <div class="control-row" data-key="w"><span>Up</span><span class="key-badge">W</span></div>
        <div class="control-row" data-key="s"><span>Down</span><span class="key-badge">S</span></div>
        <div class="control-row" data-key="a"><span>Left</span><span class="key-badge">A</span></div>
        <div class="control-row" data-key="d"><span>Right</span><span class="key-badge">D</span></div>
        <div class="control-row" data-key="e"><span>Start/ Restart</span></div>
    `,
    ghostEscape: `
        <strong>Ghost Escape</strong>
        <div class="control-row" data-key="x"><span>Deploy / Action</span><span class="key-badge">X</span></div>
    `
};

// Function to fetch and read raw binary .ch8 files
async function loadRomFile(romName) {
    try {
        const response = await fetch(`./${romName}.ch8`);
        if (!response.ok) {
            throw new Error(`Failed to locate ${romName}.ch8 file (Status: ${response.status})`);
        }
        const buffer = await response.arrayBuffer();
        return new Uint8Array(buffer);
    } catch (error) {
        console.error("Error loading the ROM file:", error);
        return null;
    }
}

async function run() {
    await init();
    
    let emulator = new Chip8();
    const gameSelect = document.getElementById('game-select');
    const colourSelect = document.getElementById('color-select');    
    const instructionsBox = document.getElementById('game-instructions');
    
    let currentRomName = gameSelect.value;
    if (instructionsBox) {
        instructionsBox.innerHTML = GAME_CONTROLS_INFO[currentRomName] || 'Use standard layout mapping configuration';
    }
    
    let romData = await loadRomFile(currentRomName);
    if (romData) {
        emulator.loadRom(romData);
        console.log(`successfully fetched and loaded: ${currentRomName}.ch8`);
    }

    // Watch for dropdown changes to load files and update side panel text dynamically
    gameSelect.addEventListener('change', async (e) => {
        currentRomName = e.target.value;
        
        // Dynamic side panel layout update
        if (instructionsBox) {
            instructionsBox.innerHTML = GAME_CONTROLS_INFO[currentRomName] || 'Use standard layout mapping configuration';
        }
        
        // Fetch the file next
        const newRomData = await loadRomFile(currentRomName);
        if (newRomData) {
            // Allocate a fresh instance to safely scrub old game variables from RAM
            emulator = new Chip8(); 
            emulator.loadRom(newRomData);
            console.log(`swapped emulator profile to: ${currentRomName}.ch8`);
        }
    });

    window.addEventListener('keydown', (e) => {
        const chip8Key = KEY_MAP[e.key.toLowerCase()];
        if (chip8Key !== undefined) {
            emulator.setKey(chip8Key, true);
        }
    });

    window.addEventListener('keyup', (e) => {
        const chip8Key = KEY_MAP[e.key.toLowerCase()];
        if (chip8Key !== undefined) {
            emulator.setKey(chip8Key, false);
        }
    });

    if (instructionsBox) {
        ['touchstart', 'mousedown'].forEach(evtType => {
            instructionsBox.addEventListener(evtType, (e) => {
                const row = e.target.closest('.control-row');
                if (!row) return;
                
                // Stop zooming or default mouse emulator lag on smartphones
                if (e.cancelable) e.preventDefault();
                
                const keyStr = row.getAttribute('data-key');
                const chip8Key = KEY_MAP[keyStr];
                if (chip8Key !== undefined) {
                    emulator.setKey(chip8Key, true);
                }
            });
        });

        ['touchend', 'mouseup', 'mouseleave'].forEach(evtType => {
            instructionsBox.addEventListener(evtType, (e) => {
                const row = e.target.closest('.control-row');
                if (!row) return;
                
                if (e.cancelable) e.preventDefault();
                
                const keyStr = row.getAttribute('data-key');
                const chip8Key = KEY_MAP[keyStr];
                if (chip8Key !== undefined) {
                    emulator.setKey(chip8Key, false);
                }
            });
        });
    }

    // main engine game loop
    function gameLoop() {
        for (let i = 0; i < 10; i++) {
            let opcode = emulator.fetch();
            emulator.decode(opcode);
        }
        emulator.updateTimer();

        // canvas ui refresh
        ctx.fillStyle = '#010409'; 
        ctx.fillRect(0, 0, canvas.width, canvas.height);
        
        ctx.fillStyle = colourSelect ? colourSelect.value : '#f0883e';
        
        const displayData = emulator.getDisplay();

        for (let i = 0; i < 2048; i++) {
            if (displayData[i] === 1) {
                const x = i % 64;
                const y = Math.floor(i / 64);
                ctx.fillRect(x * SCALE, y * SCALE, SCALE, SCALE);
            }
        }

        requestAnimationFrame(gameLoop);
    }

    requestAnimationFrame(gameLoop);
}

run();
*/


/*
import init, { Chip8 } from '../emulator-core/pkg/emulator_core.js';

const canvas = document.getElementById('chip8-canvas');
const ctx = canvas.getContext('2d');
const SCALE = 16; // Scales 64x32 to 1024x512

const KEY_MAP = {
    '1': 0x1, '2': 0x2, '3': 0x3, '4': 0xC,
    'q': 0x4, 'w': 0x5, 'e': 0x6, 'r': 0xD,
    'a': 0x7, 's': 0x8, 'd': 0x9, 'f': 0xE,
    'z': 0xA, 'x': 0x0, 'c': 0xB, 'v': 0xF
};

// side panel
const GAME_CONTROLS_INFO = {
    pong: '<strong>Pong Controls</strong><br><br><span class="key-badge">W</span> Up<br><span class="key-badge">S</span> Down<br><br><span class="key-badge">A</span> Left<br><span class="key-badge">D</span> Right',
    ghostEscape: '<strong>Ghost Escape</strong><br><br>Press <span class="key-badge">X</span> Deploy!'
};

// function to fetch and read raw binary .ch8 files
async function loadRomFile(romName) {
    try {
        const response = await fetch(`./${romName}.ch8`);
        if (!response.ok) {
            throw new Error(`Failed to locate ${romName}.ch8 file (Status: ${response.status})`);
        }
        const buffer = await response.arrayBuffer();
        return new Uint8Array(buffer);
    } catch (error) {
        console.error("Error loading the ROM file:", error);
        return null;
    }
}

async function run() {
    await init();
    
    let emulator = new Chip8();
    const gameSelect = document.getElementById('game-select');
    const colourSelect = document.getElementById('color-select');	
    const instructionsBox = document.getElementById('game-instructions');
    

    let currentRomName = gameSelect.value;
    if (instructionsBox) {
        instructionsBox.innerHTML = GAME_CONTROLS_INFO[currentRomName] || 'Use standard layout mapping configuration';
    }
    
    let romData = await loadRomFile(currentRomName);
    if (romData) {
        emulator.loadRom(romData);
        console.log(`successfully fetched and loaded: ${currentRomName}.ch8`);
    }

    // Watch for dropdown changes to load files and update side panel text dynamically
    gameSelect.addEventListener('change', async (e) => {
        currentRomName = e.target.value;
        
        // Dynamic side panel layout update
        if (instructionsBox) {
            instructionsBox.innerHTML = GAME_CONTROLS_INFO[currentRomName] || 'Use standard layout mapping configuration';
        }
        
        // Fetch the file next
        const newRomData = await loadRomFile(currentRomName);
        if (newRomData) {
            // Allocate a fresh instance to safely scrub old game variables from RAM
            emulator = new Chip8(); 
            emulator.loadRom(newRomData);
            console.log(`swapped emulator profile to: ${currentRomName}.ch8`);
        }
    });

    window.addEventListener('keydown', (e) => {
        const chip8Key = KEY_MAP[e.key.toLowerCase()];
        if (chip8Key !== undefined) {
            emulator.setKey(chip8Key, true);
        }
    });

    window.addEventListener('keyup', (e) => {
        const chip8Key = KEY_MAP[e.key.toLowerCase()];
        if (chip8Key !== undefined) {
            emulator.setKey(chip8Key, false);
        }
    });

    // Main Engine Game Loop
    function gameLoop() {
        for (let i = 0; i < 10; i++) {
            let opcode = emulator.fetch();
            emulator.decode(opcode);
        }
        emulator.updateTimer();

        // Canvas UI Refresh
        ctx.fillStyle = '#010409'; 
        ctx.fillRect(0, 0, canvas.width, canvas.height);
        
        ctx.fillStyle = colourSelect ? colourSelect.value : '#f0883e';
        
        const displayData = emulator.getDisplay();

        for (let i = 0; i < 2048; i++) {
            if (displayData[i] === 1) {
                const x = i % 64;
                const y = Math.floor(i / 64);
                ctx.fillRect(x * SCALE, y * SCALE, SCALE, SCALE);
            }
        }

        requestAnimationFrame(gameLoop);
    }

    requestAnimationFrame(gameLoop);
}

run();
*/

/*
import init, { Chip8 } from '../emulator-core/pkg/emulator_core.js';

const canvas = document.getElementById('chip8-canvas');
const ctx = canvas.getContext('2d');
const SCALE = 16; // 64x32 grid up to 16 times more

const KEY_MAP = {
    '1': 0x1, '2': 0x2, '3': 0x3, '4': 0xC,
    'q': 0x4, 'w': 0x5, 'e': 0x6, 'r': 0xD,
    'a': 0x7, 's': 0x8, 'd': 0x9, 'f': 0xE,
    'z': 0xA, 'x': 0x0, 'c': 0xB, 'v': 0xF
};

async function run() {
    await init();
    
    const emulator = new Chip8();
    console.log("wasm emulator core loaded");

    try {

        const response = await fetch('./pong.ch8');
        const buffer = await response.arrayBuffer();
        const romData = new Uint8Array(buffer);
    
        
        emulator.loadRom(romData);
        console.log("rom loaded successfully");
        
        window.addEventListener('keydown', (e) => {
            const chip8Key = KEY_MAP[e.key.toLowerCase()];
            if (chip8Key !== undefined) {
                emulator.setKey(chip8Key, true);
            }
        });

        window.addEventListener('keyup', (e) => {
            const chip8Key = KEY_MAP[e.key.toLowerCase()];
            if (chip8Key !== undefined) {
                emulator.setKey(chip8Key, false);
            }
        });

        // game Engine Loop
        function gameLoop() {
            for (let i = 0; i < 10; i++) {
                let opcode = emulator.fetch();
                emulator.decode(opcode);
            }
            emulator.updateTimer();

            ctx.fillStyle = 'black';
            ctx.fillRect(0, 0, canvas.width, canvas.height);
            
            ctx.fillStyle = 'white';
            
            const displayData = emulator.getDisplay();

            for (let i = 0; i < 2048; i++) {
                if (displayData[i] === 1) {
                    const x = i % 64;
                    const y = Math.floor(i / 64);
                    ctx.fillRect(x * SCALE, y * SCALE, SCALE, SCALE);
                }
            }

            requestAnimationFrame(gameLoop);
        }

        requestAnimationFrame(gameLoop);
        
    } catch (err) {
        console.error("error running game loop:", err);
    }
}

run();
*/

