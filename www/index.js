import { Universe } from "wasm-game-of-life";
import { memory } from "wasm-game-of-life/gol_rust_bg";
import { PATTERN } from "./pattern";

const CELL_SIZE = 2;
const LIVE_RGBA = [244, 43, 3, 255];

const universe = Universe.new(window.innerWidth / CELL_SIZE, window.innerHeight / CELL_SIZE, PATTERN);
const width = universe.width();
const height = universe.height();

const canvas = document.getElementById("game-of-life-canvas");
canvas.width = width * CELL_SIZE;
canvas.height = height * CELL_SIZE;

const ctx = canvas.getContext('2d');

const bitIsSet = (i, arr) => {
    const byte = Math.floor(i / 8);
    const mask = 1 << (i % 8);
    return (arr[byte] & mask) === mask;
};

const drawCells = () => {
  const cells = new Uint8Array(memory.buffer, universe.cells(), width * height / 8);
  const cellRGB = new Uint8ClampedArray(canvas.width * canvas.height * 4);

  for (let i = 0; i < width * height; i++) {
    if (bitIsSet(i, cells)) {
      const x = i % width;
      const y = Math.floor(i / width);

      for (let dy = 0; dy < CELL_SIZE; dy++) {
        for (let dx = 0; dx < CELL_SIZE; dx++) {
          const baseIndex = ((y * CELL_SIZE + dy) * canvas.width + (x * CELL_SIZE + dx)) * 4;
          cellRGB[baseIndex] =  LIVE_RGBA[0];
          cellRGB[baseIndex + 1] = LIVE_RGBA[1];
          cellRGB[baseIndex + 2] = LIVE_RGBA[2];
          cellRGB[baseIndex + 3] = LIVE_RGBA[3];
        }
      }
    }
  }

  let imageData = new ImageData(cellRGB, canvas.width, canvas.height);
  ctx.putImageData(imageData, 0, 0);
};

const renderLoop = () => {
    universe.tick();

    drawCells();

    requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);
