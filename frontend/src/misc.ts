export function failTimeToString(failTimeSeconds: number): string {
	let hours = Math.floor(failTimeSeconds/3600);
	let hoursLeft = hours % 24;
	let days = Math.floor(hours/24);

	return `Failed after ${days} days and ${hoursLeft} hours.`
}

function timeLeft(diff: number): {days: number, hours: number, minutes: number, seconds: number} {
	let days = Math.floor(diff/3600/24);
	let hours = Math.floor(diff/3600 % 24);
	let minutes = Math.floor(diff/60 % 60);
	let seconds = diff % 60;
	return {
		days: days,
		hours: hours,
		minutes: minutes,
		seconds: seconds
	}
}

export function getTimer(diff: number): string {
	let time = timeLeft(diff);
	return `${time.days.toString().padStart(2, "0")}:${time.hours.toString().padStart(2, "0")}:${time.minutes.toString().padStart(2, "0")}:${time.seconds.toString().padStart(2, "0")}`;
}

export function pressedEnter(event, callback) {
	if (event.key === "Enter") callback();
}