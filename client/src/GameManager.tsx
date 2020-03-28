import * as React from "react";
import {Game} from "./Game";

const SERVER_URL = "";

export class GameManager extends React.Component<GameManagerProps, GameManagerState> {
    private ws: WebSocket;

    constructor(props: GameManagerProps) {
        super(props);
        this.ws = new WebSocket(SERVER_URL);
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
            const message = JSON.parse(event.data);

            switch(message.type) {
                case 'map':
                    break;
                case 'patch':
                    break;
                default:
                    break;
            }
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

export interface GameManagerState {}

export interface GameManagerProps {}

