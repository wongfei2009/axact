import { h, Component, render } from 'https://esm.sh/preact';

setInterval(async function () {
    let response = await fetch("/api/cpu");
    let data = await response.json();
    const app = h('pre', {}, JSON.stringify(data, null, 2));
    render(app, document.body);
}, 1000);