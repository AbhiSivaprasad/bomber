const BOARD_HEIGHT = 400;
const BOARD_WIDTH = 400;
const CELL_SIZE = 40;
const OFFSET = 20;

const canvas = document.getElementById("canvas");
const context = canvas.getContext("2d");

/**
 * draw grid of height BOARD_HEIGHT, width BOARD_WIDTH, and square cells with side CELL_SIZE
 * position grid at an offset OFFSET relative to canvas origin
 */
function drawBoard() {
    // vertical lines
    for (let x = 0; x <= BOARD_WIDTH; x += CELL_SIZE) {
        context.moveTo(0.5 + x + OFFSET, OFFSET);
        context.lineTo(0.5 + x + OFFSET, BOARD_HEIGHT + OFFSET);
    }

    // horizontal lines
    for (let y = 0; y <= BOARD_HEIGHT; y += CELL_SIZE) {
        context.moveTo(OFFSET, 0.5 + y + OFFSET);
        context.lineTo(BOARD_WIDTH + OFFSET, 0.5 + y + OFFSET);
    }

    context.strokeStyle = "black";
    context.stroke();
}

drawBoard();