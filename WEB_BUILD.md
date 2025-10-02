# Web-based 3D Visualization

For environments where native window display is not available (e.g., containers, remote servers), you can build a web version.

## Building for Web (WASM)

1. Install the wasm32 target:
```bash
rustup target add wasm32-unknown-unknown
```

2. Install basic-http-server:
```bash
cargo install basic-http-server
```

3. Build the WASM version:
```bash
cargo build --release --target wasm32-unknown-unknown
```

4. Copy the build artifacts:
```bash
mkdir -p web
cp target/wasm32-unknown-unknown/release/grid_simulation.wasm web/
```

5. Create an index.html file (see web/index.html)

6. Serve it:
```bash
basic-http-server web
```

Then open your browser to `http://localhost:4000`

## Current Status

The native desktop version is fully functional and provides the best performance.
The web version can be added as an enhancement for better accessibility.
