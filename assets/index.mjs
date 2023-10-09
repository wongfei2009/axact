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

async function renderCpuUsage() {
    try {
        let response = await fetch("/api/cpu");
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }
        let data = await response.json();
        render(html`<${App} cpus=${data}></${App}>`, document.body);
    } catch (error) {
        console.error(error);
    }
}

setInterval(renderCpuUsage, 200);
