import { h, render } from 'https://esm.sh/preact';
import htm from 'https://esm.sh/htm';

const html = htm.bind(h);

function App(props) {
    return html`
    <div>
        ${props.cpus.map((cpu) => {
        return html`
            <div class="bar">
                <div class="bar-inner" style="width: ${cpu}%"></div>
                <label>${cpu.toFixed(2)}%</label>
            </div>
            `;
    })}
    </div>  
    `;
}

const ws = new WebSocket(`ws://${window.location.host}/api/realtime-cpu`);
ws.onmessage = (event) => {
    const data = JSON.parse(event.data);
    render(html`<${App} cpus=${data}></${App}>`, document.body);
};