import './app.css'
// @ts-ignore
import App from './App.svelte'

const app = new App({
	target: document.getElementById('app'),
})

customElements.define("delayed-content", class extends HTMLElement {
	connectedCallback() {
		setTimeout(() => {
			this.removeAttribute("hidden");
		}, Number(this.getAttribute("delay")));
	}
});

export default app
