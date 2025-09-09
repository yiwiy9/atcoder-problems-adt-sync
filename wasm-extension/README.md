# AtCoder Problems ADT Sync Extension

Chrome Extension to merge AC submissions from AtCoder Daily Training (ADT) into [AtCoder Problems](https://kenkoooo.com/atcoder/).

## Build

```bash
wasm-pack build --target web --out-dir ./extension/pkg
```

> **Note:**  
> Environment variables can be configured in `.env.frontend`.  
> These values are embedded at build time and should not contain secrets.  
> See `.env.frontend.example` for reference.

## Load Extension in Chrome

1. Open `chrome://extensions/` in Chrome  
2. Enable "Developer mode"  
3. Click "Load unpacked"  
4. Select the `extension/` directory

## Dev Container (VSCode)

This directory supports development using VSCode Dev Containers.

1. Open this `wasm-extension/` directory in VSCode  
2. Reopen in Container (requires Remote - Containers extension)

## License

MIT License
