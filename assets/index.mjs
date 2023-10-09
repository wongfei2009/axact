import { h, render } from 'https://esm.sh/preact';
import htm from 'https://esm.sh/htm';

const html = htm.bind(h);

function App(props) {
    return html`
    <div>
        ${props.cpus.map((cpu) => {
        return html`
            <div>
                <p>${cpu.toFixed(2)}%</p>
            </div>
            `;
    })}
    </div>
    `;
}

setInterval(async function () {
    let response = await fetch("/api/cpu");
    let data = await response.json();
    render(html`<${App} cpus=${data}></${App}>`, document.body);
}, 1000);