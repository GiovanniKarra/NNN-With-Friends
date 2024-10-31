<script>
	import { onDestroy } from "svelte";
	import { getTimeInterval } from "./api";
	import { getTimer } from "./misc";

	let timeInterval = [0, 0];
	getTimeInterval().then((res) => timeInterval = res);

	let currentTime = Math.floor(Date.now()/1000);
	const interval = setInterval(() => {
		currentTime = Math.floor(Date.now()/1000);
	}, 1000);
	onDestroy(() => clearInterval(interval));

	let displayText = "";
	let timer = "";
	let diff = 0;
	$: {
		if (currentTime < timeInterval[0]) {
			diff = timeInterval[0]-currentTime;
			displayText = "Time left until No Nut November";
		}
		else if (currentTime < timeInterval[1]) {
			diff = timeInterval[1]-currentTime;
			displayText = "Time left until the end of No Nut November";
		}
		else {
			diff = currentTime-timeInterval[1];
			displayText = "Time since the end of No Nut November";
		}
		timer = getTimer(diff);
	}
</script>

<div class="timer">
	<h3>{displayText}</h3>
	<h2>{timer}</h2>
</div>