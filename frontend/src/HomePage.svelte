<script>
    import { onDestroy } from "svelte";
    import { getTimeInterval } from "./api";
    import { timeLeft } from "./misc";

	let timeInterval = [0, 0];
	getTimeInterval().then((res) => timeInterval = res);

	let currentTime = Math.floor(Date.now()/1000);
	const interval = setInterval(() => currentTime = Math.floor(Date.now()/1000), 1000);
	onDestroy(() => clearInterval(interval));

	let displayText = "";
	$: {
		if (currentTime < timeInterval[0]) {
			let diff = timeInterval[0]-currentTime;
			let time = timeLeft(diff);
			displayText = `Time left until No Nut November\n
				${time.days}:${time.hours}:${time.minutes}:${time.seconds}`;
		}
		else if (currentTime < timeInterval[1]) {
			let diff = timeInterval[1]-currentTime;
			let time = timeLeft(diff);
			displayText = `Time left until the end of No Nut November\n
				${time.days}:${time.hours}:${time.minutes}:${time.seconds}`;
		}
		else {
			let diff = currentTime-timeInterval[1];
			let time = timeLeft(diff);
			displayText = `Time since the end of No Nut November\n
				${time.days}:${time.hours}:${time.minutes}:${time.seconds}`;
		}
	}
</script>


<h1>HOME PAGE</h1>
<p>{displayText}</p>