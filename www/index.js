import { Universe } from "wasm-game-of-life";
import { memory } from "wasm-game-of-life/gol_rust_bg";

const CELL_SIZE = 4;
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

const universe = Universe.new(window.innerWidth / CELL_SIZE, window.innerHeight / CELL_SIZE);
const width = universe.width();
const height = universe.height();

const canvas = document.getElementById("game-of-life-canvas");
canvas.width = CELL_SIZE * width;
canvas.height = CELL_SIZE * height;

const ctx = canvas.getContext('2d');

const getIndex = (x, y) => {
    return y * width + x;
};

const bitIsSet = (i, arr) => {
    const byte = Math.floor(i / 8);
    const mask = 1 << (i % 8);
    return (arr[byte] & mask) === mask;
};

const drawCells = () => {
    const cells = new Uint8Array(memory.buffer, universe.cells(), width * height / 8);

    ctx.beginPath();
  
    for (let x = 0; x < width; x++) {
      for (let y = 0; y < height; y++) {
        const i = getIndex(x, y);
  
        ctx.fillStyle = bitIsSet(i, cells)
          ? ALIVE_COLOR
          : DEAD_COLOR;
  
        ctx.fillRect(
          x * (CELL_SIZE + 1) + 1,
          y * (CELL_SIZE + 1) + 1,
          CELL_SIZE,
          CELL_SIZE
        );
      }
    }
  
    ctx.stroke();
};

const renderLoop = () => {
    universe.tick();

    drawCells();

    requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);
