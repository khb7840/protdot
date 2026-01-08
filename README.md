# protdot

Interactive 3D protein structure viewer built with Rust and Macroquad.

## Quick Start

### Desktop
```bash
cargo build --release
./target/release/protdot ./data/1G2F.pdb
```

### Web
```bash
./build_wasm.sh
cd docs && python3 -m http.server 8000
```

## Controls

Press **H** in the app to toggle the help overlay with all keybindings.

### Essentials
- **Mouse drag**: Rotate camera
- **Scroll**: Zoom
- **C**: Cycle color schemes
- **T**: Toggle animation
- **R**: Reset view

## Features

- Multiple color schemes (element, amino acid, gradients, themes)
- Various animation modes with reverse direction
- Model rotation in screen space (WASD/QE keys)
- Export to PNG and SVG (desktop only)
- Custom color palette support
- Per-atom and per-residue rendering modes
