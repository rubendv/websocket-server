const ws = new WebSocket("ws://127.0.0.1:8080");
const points_el = document.getElementById("points");

const values = [];

ws.onmessage = (ev) => {
    const message = document.createElement("p");
    message.innerText = ev.data;
    values.push(JSON.parse(ev.data));

    let min = Math.min(...values);
    let max = Math.max(...values);
    let range = max - min;
    let scale = 300 / range;

    console.log("min: " + min + ", max: " + max);

    let points = "M 0 " + (300 - Math.round((values[0] - min) * scale));
    for (let i = 0; i < values.length; i++) {
        points += " L " + Math.round(i * 1200 / values.length) + ", " + (300 - Math.round((values[i] - min) * scale));
    }

    points_el.setAttribute("d", points);
};
