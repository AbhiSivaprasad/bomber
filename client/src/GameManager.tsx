import * as React from "react";
import {Game} from "./Game";
import {PlayerObject} from "./types";

const SERVER_URL = "";

export class GameManager extends React.Component<GameManagerProps, GameManagerState> {
    private ws: WebSocket;

    constructor(props: GameManagerProps) {
        super(props);
        this.ws = new WebSocket(SERVER_URL);

        // initialize to empty state with no players
        this.state = {
            terrain: null,
            generals: [],
            mines: [],
            minions: [],
            bullets: [],
        };
    }

    render() {
        return <Game />
    }

    componentDidMount() {
        this.ws.onopen = () => {
            console.log("Connected to Server");
        };

        this.ws.onmessage = (event) => {
            console.log(`Data received from server: ${event.data}`);
            this.setState(JSON.parse(event.data))
        };

        this.ws.onclose = (event) => {
            if (event.wasClean) {
                console.log(`[close] Connection closed cleanly, code=${event.code} reason=${event.reason}`);
            } else {
                // e.g. server process killed or network down
                // event.code is usually 1006 in this case
                console.log('[close] Connection died');
            }
        };

        this.ws.onerror = (event) => {
            console.error(`[error] ${event}`);
        };
    }
}

export interface GameManagerState {
    terrain: number[][];

    // objects attached to a player and location
    generals: PlayerObject[];
    mines: PlayerObject[];
    minions: PlayerObject[];
    bullets: PlayerObject[];
}

export interface GameManagerProps {}

