export async function getStatus(username: String): Promise<JSON> {
	let res = await fetch(`/api/users/${username}/status`);
	return await res.json();
}

export async function getGroupStatus(groupid: String): Promise<JSON> {
	let res = await fetch(`/api/groups/${groupid}/status`);
	return await res.json();
}

export async function createGroup(groupName: String): Promise<JSON> {
	let res = await fetch(`/api/groups/create/${groupName}`, {method: "POST"});
	return await res.json();
}

export async function addToGroup(groupid: String, username: String): Promise<JSON> {
	let res = await fetch(`/api/groups/${groupid}/add/${username}`, {method: "POST"});
	return await res.json();
}

export async function joinGroup(groupid: String): Promise<JSON> {
	let res = await fetch(`/api/groups/${groupid}/join`, {method: "POST"});
	return await res.json();
}

export async function getMyGroups(): Promise<JSON> {
	let res = await fetch(`/api/groups/myGroups`);
	return await res.json();
}

export async function fail(message: String): Promise<JSON> {
	let res = await fetch(`/api/users/fail/${message}`);
	return await res.json();
}