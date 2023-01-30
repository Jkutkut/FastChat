//Websocekt variables
const url = `ws://${window.location.hostname}:4242/ws/` // TODO fill

window.addEventListener('load', () => {
    const mywsServer = new WebSocket(url);

    //DOM Elements
    const myMessages = document.getElementById("messages");
    const myInput = document.getElementById("message");
    const sendBtn = document.getElementById("send");


    const add2chat = (msgObj) => {
        const {user, msg} = JSON.parse(msgObj);
        const newMessage = document.createElement("p");
        newMessage.innerHTML = `<b>${user}</b>: ${msg}`;
        myMessages.appendChild(newMessage);
    }

    sendBtn.disabled = true;
    sendBtn.addEventListener("click", () => {
        const text = myInput.value;
        mywsServer.send(text);
    });

    mywsServer.onopen = () => {
        sendBtn.disabled = false;
    }

    mywsServer.onclose = () => {
        sendBtn.disabled = true;
    }

    mywsServer.onmessage = (event) => {
        const { data } = event;
        add2chat(data);
    }
});