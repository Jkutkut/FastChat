const WebSocket = require("ws");
const express = require("express");
const app = express();
const path = require("path");

const PORT = 4242;

// **** View engine setup ****
app.use(express.static(path.join(__dirname, 'webclient')));
// app.set('views', path.join(__dirname, 'views'));
// app.set('view engine', 'hbs');

// **** Websocket server ****
const wsServer = new WebSocket.Server({
    noServer: true
});

wsServer.on("connection", (ws) => {
    console.log("New client connected. Total clients: " + wsServer.clients.size);
    wsServer.clients.forEach((client) => {
        if (client.readyState === WebSocket.OPEN) {
            client.send("New client connected. Total clients: " + wsServer.clients.size);
        }
    });
    
    ws.on("message", (msg) => {
        wsServer.clients.forEach((client) => {
            if (client.readyState === WebSocket.OPEN) {
              client.send(msg.toString());
            }
        });
    });
    ws.on("error", console.error);
    ws.on("close", () => {
        console.log("Client disconnected. Total clients: " + wsServer.clients.size);
        wsServer.clients.forEach((client) => {
            if (client.readyState === WebSocket.OPEN) {
                client.send("Client disconnected. Total clients: " + wsServer.clients.size);
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

