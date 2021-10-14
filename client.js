const ws = new WebSocket("ws://127.0.0.1:8080");
const messages = document.getElementById("messages");

ws.onmessage = (ev) => {
    const message = document.createElement("p");
    message.innerText = ev.data;
    messages.appendChild(message);
};

ws.onopen = (ev) => {
    ws.send("Test");
    setTimeout(ws.onopen, 1000);
};
