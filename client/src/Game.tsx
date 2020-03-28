import * as React from "react";
import {PlayerObject} from "./types";

const BOARD_HEIGHT = 400;
const BOARD_WIDTH = 400;
const CELL_SIZE = 40;
const OFFSET = 20;

export class Game extends React.Component<GameProps, {}> {
    private canvasRef: React.RefObject<HTMLCanvasElement>;

    constructor(props: GameProps) {
        super(props);

        this.canvasRef = React.createRef();
    }


    render() {
        const style = {
            background: "#fff",
            margin: "20px",
        };

        return <canvas ref={this.canvasRef} width={500} height={500} style={style} />;
    }

    componentDidMount() {
        const ctx = this.canvasRef.current.getContext("2d");
        Game.drawBoard(ctx);
    }

    static drawBoard(context: CanvasRenderingContext2D) {
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
}

export interface GameState {
    generals: PlayerObject[];
    mines: PlayerObject[];
    minions: PlayerObject[];
    bullets: PlayerObject[];
    terrain: number[][];
}

export interface GameProps { }
