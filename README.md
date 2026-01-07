# Protein Viewer - WASM Web App

A 3D protein structure viewer built with Rust and Macroquad, compiled to WebAssembly.

## Features

- 🔄 Smooth rotation with momentum
- 🎨 Color by element or amino acid residue
- 🎭 Sphere or backbone ribbon rendering modes
- 🖼️ PNG export
- 🖱️ Mouse controls (rotate, pan, zoom)
- 🌐 Runs in the browser via WebAssembly

## Building for Web

### Prerequisites

- Rust toolchain
- wasm32-unknown-unknown target

### Build Steps

1. Run the build script:
```bash
./build_wasm.sh
```

2. Serve the web directory:
```bash
cd web
python3 -m http.server 8000
```

3. Open your browser to `http://localhost:8000`

## Controls

- **Left Mouse Drag**: Rotate the protein
- **Middle Mouse Drag**: Pan the view
- **Mouse Scroll**: Zoom in/out
- **C Key**: Toggle color mode (Element/Residue)
- **B Key**: Toggle render mode (Spheres/Backbone Ribbon)
- **E Key**: Export current view as PNG

## Building for Desktop

```bash
cargo build --release
./target/release/protein_viewer [path/to/protein.pdb]
```

## Color Schemes

### By Element
- Carbon: Dark gray
- Oxygen: Soft red
- Nitrogen: Soft blue
- Sulfur: Golden
- Hydrogen: Light gray

### By Residue
- Hydrophobic: Beige/tan
- Polar: Green
- Positive charge: Blue
- Negative charge: Red
- Glycine: White

## Technical Details

- Built with [Macroquad](https://github.com/not-fl3/macroquad)
- Compiled to WebAssembly using `wasm32-unknown-unknown` target
- PDB file parsing with structure centering
- Real-time 3D rendering with WebGL
