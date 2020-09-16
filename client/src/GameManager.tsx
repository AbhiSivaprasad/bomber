import * as React from "react";
import {Game, GameState} from "./Game";

const SERVER_URL = "";

export class GameManager extends React.Component<GameManagerProps, GameManagerState> {
    private ws: WebSocket;

    constructor(props: GameManagerProps) {
        super(props);
        this.ws = new WebSocket(SERVER_URL);

        // initialize to empty state with no players
        this.state = {
            terrain: [[0]],
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
            // TODO: parse terrain correctly
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
    gameState: GameState,
    // objects attached to a player and location
}

export interface GameManagerProps {}

