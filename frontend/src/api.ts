export async function getStatus(username: string): Promise<{failed: boolean, failed_time: number, failed_msg: string}> {
	let res = await fetch(`/api/users/${username}/status`);
	let resJSON = await res.json();
	return {
		failed: resJSON.failed_time !== null,
		failed_time: resJSON.failed_time,
		failed_msg: resJSON.failed_msg
	}
}

export async function getGroupStatus(groupid: string): Promise<Array<{username: string, failed_time: number, failed_msg: string}>> {
	let res = await fetch(`/api/groups/${groupid}/status`);
	return await res.json();
}

export async function createGroup(groupName: string): Promise<{success: boolean, groupid: string}> {
	let res = await fetch(`/api/groups/create/${groupName}`, {method: "POST"});
	return {success: res.ok, groupid: await res.text()}
}

export async function addToGroup(groupid: string, username: string): Promise<{success: boolean, message: string}> {
	let res = await fetch(`/api/groups/${groupid}/add/${username}`, {method: "POST"});
	return {success: res.ok, message: await res.text()}
}

export async function joinGroup(groupid: string): Promise<{success: boolean, message: string}> {
	let res = await fetch(`/api/groups/${groupid}/join`, {method: "POST"});
	return {success: res.ok, message: await res.text()}
}

export async function leaveGroup(groupid: string) {
	await fetch(`/api/groups/${groupid}/leave`, {method: "POST"});
}

export async function getMyGroups(): Promise<Array<{id: string, name: string, founder: string}>> {
	let res = await fetch(`/api/groups/myGroups`);
	console.log("yep, it was used");
	return await res.json();
}

export async function fail(message: string): Promise<{failed: boolean, failed_time: number, failed_msg: string}> {
	let res = await fetch(`/api/users/fail/${message}`, {method: "POST"});
	let resJSON = await res.json();
	return {
		failed: resJSON.failed_time !== undefined,
		failed_time: resJSON.failed_time,
		failed_msg: resJSON.failed_msg
	}
}

export async function getTimeInterval(): Promise<[number, number]> {
	let res = await fetch(`/api/timeWindow`);
	let interval = await res.json();
	let offset = new Date().getTimezoneOffset();
	interval[0] += offset*60;
	interval[1] += offset*60;
	return interval;
}