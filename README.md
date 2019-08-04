# wasm-flood-fill

Implementation of a flood fill algorithm in Rust for WebAssembly.

This is an experimentation with WebAssembly to speed-up CPU intensive operations (flood-fill on large canvas).


## Usage 

```typescript
import * as wasm from 'wasm-flood-fill'

const startX: number;
const startY: number;
const color: {r, g, b}: {r: number, g: number, b: number};
const context: CanvasRenderingContext2D;
const {width, height} = context.canvas;
const imageData = context.getImageData(0, 0, width, height);

const data = wasm.flood_fill(
  context, 
  imageData.data, 
  Math.round(startX), Math.round(startY), // Important that it is rounded before passing it to wasm
  r, g, b,  
  50 // tolerance (0-255 range)
);

context.putImageData(new ImageData(data, width, height), 0, 0);
```