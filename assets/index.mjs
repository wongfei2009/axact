import { h, render } from 'https://esm.sh/preact';
import htm from 'https://esm.sh/htm';

const html = htm.bind(h);

function App(props) {
    return html`
    <div>
        ${props.cpus.map((cpu) => {
        return html`
            <div class="bar">
                <div class="bar-inner" style="width: ${cpu}%"> ${cpu.toFixed(2)}%</div>
            </div>
            `;
    })}
    </div>  
    `;
}

async function renderCpuUsage() {
    let response = await fetch("/api/cpu");
    let data = await response.json();
    render(html`<${App} cpus=${data}></${App}>`, document.body);
}

setInterval(renderCpuUsage, 200);
