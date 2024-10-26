import { pageState } from "./state";


export async function login(username: string, password: string): Promise<string> {
	let res = username === ""?
	await fetch(`/api/login`, { 
		method: "POST",
		headers: {"content-type": "application/json"} }
	):
	await fetch(`/api/login`, { 
		method: "POST",
		body: JSON.stringify({ username: username, password: password}),
		headers: {"content-type": "application/json"} }
	)
	let jsonRes = await res.json();

	if (jsonRes.success === false) {
		pageState.update(
			(current) => ({...current, user: "" })
		);
		return jsonRes.message;
	} else {
		pageState.update(
			(current) => ({...current, user: jsonRes.user })
		);
		return "";
	}
}

export async function signup(username: string, password: string): Promise<string> {
	let res = await fetch(`/api/signup`, { 
		method: "POST",
		body: JSON.stringify({ username: username, password: password}),
		headers: {"content-type": "application/json"} }
	)
	let jsonRes = await res.json();

	if (jsonRes.success === false) {
		return jsonRes.message;
	} else {
		return "";
	}
}


export async function logout() {
	let res = await fetch("/api/logout", {method: "POST"});
	return res;
}