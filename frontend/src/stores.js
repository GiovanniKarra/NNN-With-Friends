import { writable } from "svelte/store";

let state = window.location.pathname.replace(/^\//, "").replace(/\/$/, "");
let path = state === ""? ["home"] : state.split("/")
export const pageState = writable({path: path});