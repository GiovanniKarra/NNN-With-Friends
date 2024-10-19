import { writable } from "svelte/store";


export function get_state_from_url() {
	let pathString = window.location.pathname.replace(/^\//, "").replace(/\/$/, "");
	let path = pathString === ""? ["home", ""] : pathString.split("/");
	let page = path[0]; let arg = path[1];
	return {"page": page, "arg": arg === undefined ? "": arg};
}

export const pageState = writable(get_state_from_url());
pageState.subscribe((newState) => {
	window.history.pushState(null, null, `/${newState.page}/${newState.arg}`)
})