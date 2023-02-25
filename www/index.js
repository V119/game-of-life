import { Universe, Cell } from "game-of-life";
import { memory } from "game-of-life/life_game_bg"

const CELL_SIZE = 5;
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

const universe = Universe.new(128 * 2, 128 * 2);
const width = universe.width();
const height = universe.height();

const canvas = document.getElementById("game-of-life-canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const ctx = canvas.getContext("2d");

const drawGrid = () => {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;

    for (let i = 0; i <= width; i++) {
        ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
        ctx.lineTo(i * (CELL_SIZE + 1) + 1, height * (CELL_SIZE + 1) + 1);
    }

    for (let i = 0; i <= height; i++) {
        ctx.moveTo(0, i * (CELL_SIZE + 1) + 1);
        ctx.lineTo(width * (CELL_SIZE + 1) + 1, i * (CELL_SIZE + 1) + 1);
    }

    ctx.stroke();
}

const getIdx = (x, y) => {
    return y * width + x;
}

const drawCells = () => {
    const cellPrt = universe.cells();
    const cells = new Uint8Array(memory.buffer, cellPrt, width * height);

    ctx.beginPath();

    for (let x = 0; x < width; x++) {
        for (let y = 0; y < height; y++) {
            const idx = getIdx(x, y);
            ctx.fillStyle = cells[idx] === Cell.Dead ? DEAD_COLOR : ALIVE_COLOR;
            ctx.fillRect(
                x * (CELL_SIZE + 1) + 1,
                y * (CELL_SIZE + 1) + 1,
                CELL_SIZE,
                CELL_SIZE
            );
        }
    }

    ctx.stroke();
}
const renderLoop = () => {
    universe.tick();

    drawGrid();
    drawCells();

    requestAnimationFrame(renderLoop);
}

drawGrid();
drawCells();
requestAnimationFrame(renderLoop);
