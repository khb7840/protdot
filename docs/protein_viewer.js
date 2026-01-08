// WASM loader for macroquad
const memory = new WebAssembly.Memory({ initial: 256, maximum: 512 });

const importObject = {
    env: {
        memory: memory,
    }
};

async function loadWasm() {
    const response = await fetch('protein_viewer.wasm');
    const bytes = await response.arrayBuffer();
    const result = await WebAssembly.instantiate(bytes, importObject);
    
    // Macroquad handles the rest
    const canvas = document.getElementById('glcanvas');
    if (canvas) {
        canvas.focus();
    }
}

loadWasm().catch(console.error);
