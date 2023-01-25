//Websocekt variables
const url = "ws://10.34.177.197:4242/myWebsocket";

window.addEventListener('load', () => {
    const mywsServer = new WebSocket(url);

    //DOM Elements
    const myMessages = document.getElementById("messages");
    const myInput = document.getElementById("message");
    const sendBtn = document.getElementById("send");


    const add2chat = (msg, from) => {
        const newMessage = document.createElement("p");
        newMessage.innerHTML = `<b>${from}</b>: ${msg}`;
        myMessages.appendChild(newMessage);
    }

    sendBtn.disabled = true;
    sendBtn.addEventListener("click", () => {
        const text = myInput.value;
        add2chat(text, "Client");
        mywsServer.send(text);
    });

    //enabling send message when connection is open
    mywsServer.onopen = () => {
        sendBtn.disabled = false;
    }

    //handling message event
    mywsServer.onmessage = (event) => {
        const { data } = event
        add2chat(data, "Server")
    }
});