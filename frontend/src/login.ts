import { pageState } from "./state";


export async function login(username: String, password: String) {
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
		return jsonRes.message;
	} else {
		pageState.update(
			(current) => ({...current, path: ["home"], user: jsonRes.user })
		);
		return "";
	}
}