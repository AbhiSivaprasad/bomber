SERVER_URL = "";

let socket = new WebSocket(SERVER_URL);

socket.onopen = function(e) {
    console.log("Connected to Server");
};

socket.onmessage = function(event) {
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

socket.onclose = function(event) {
    if (event.wasClean) {
        console.log(`[close] Connection closed cleanly, code=${event.code} reason=${event.reason}`);
    } else {
        // e.g. server process killed or network down
        // event.code is usually 1006 in this case
        console.log('[close] Connection died');
    }
};

socket.onerror = function(error) {
    console.error(`[error] ${error.message}`);
};
