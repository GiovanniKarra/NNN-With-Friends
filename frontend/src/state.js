import { writable } from "svelte/store";


export function get_state_from_url() {
	let pathString = window.location.pathname.replace(/^\//, "").replace(/\/$/, "");
	let path = pathString === ""? ["home", ""] : pathString.split("/");
	let page = path[0]; let arg = path[1];
	return {"page": page, "arg": arg === undefined ? "": arg};
}

let currentState = get_state_from_url()
window.history.replaceState(null, null, `/${currentState.page}/${currentState.arg}`)

export const pageState = writable(currentState);
pageState.subscribe((newState) => {
	const newPath = `/${newState.page}/${newState.arg}`;
	window.history.replaceState(null, null, newPath);
})