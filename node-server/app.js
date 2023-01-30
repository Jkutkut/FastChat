const WebSocket = require("ws");
const express = require("express");
const app = express();
const path = require("path");

const PORT = 4000;

// **** View engine setup ****
app.use(express.static(path.join(__dirname, 'webclient')));
// app.set('views', path.join(__dirname, 'views'));
// app.set('view engine', 'hbs');

// **** Websocket server ****
const wsServer = new WebSocket.Server({
    noServer: true
});

wsServer.on("connection", (ws) => {
    const names = ['Alpha', 'Bravo', 'Charlie', 'Delta', 'Echo', 'Foxtrot', 'Golf', 'Hotel', 'India', 'Juliet', 'Kilo', 'Lima', 'Mike', 'November', 'Oscar', 'Papa', 'Quebec', 'Romeo', 'Sierra', 'Tango', 'Uniform', 'Victor', 'Whiskey', 'X-ray', 'Yankee', 'Zulu'];
    const name = names[Math.floor(Math.random() * names.length)];
    console.log(`${name} connected. Total clients: ${wsServer.clients.size}`);
    // console.log(ws); // TODO investigate ws object

    console.log(`${name} connected. Total clients: ${wsServer.clients.size}`);
    wsServer.clients.forEach((client) => {
        if (client.readyState === WebSocket.OPEN) {
            client.send(JSON.stringify({user: 'Server', msg: `${name} connected. Total clients: ${wsServer.clients.size}`}));
        }
    });
    
    ws.on("message", (msgStream) => {
        const msg = msgStream.toString();
        if (msg === "")
            return;
        wsServer.clients.forEach((client) => {
            if (client.readyState === WebSocket.OPEN) {
                client.send(JSON.stringify({user: name, msg}));
            }
        });
    });
    ws.on("error", console.error);
    ws.on("close", () => {
        console.log("Client disconnected. Total clients: " + wsServer.clients.size);
        wsServer.clients.forEach((client) => {
            if (client.readyState === WebSocket.OPEN) {
                client.send(JSON.stringify({user: 'Server', msg: `${name} disconnected. Total clients: ${wsServer.clients.size}`}));
            }
        });
    });
});

const myServer = app.listen(PORT);
console.log(`Server listening on port ${PORT}`);

myServer.on('upgrade', async (request, socket, head) => { // http to websocket
    // return socket.end("HTTP/1.1 401 Unauthorized\r\n", "ascii"); //proper connection close in case of rejection
    wsServer.handleUpgrade(request, socket, head, (ws) => {
      wsServer.emit('connection', ws, request);
    });
});

