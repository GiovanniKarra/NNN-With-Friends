<script>
	import { onDestroy } from "svelte";
	import { getTimer } from "./misc";
    import { pageState } from "./state";

	let timeInterval = $pageState.interval;

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
	{#if timeInterval[0] !== 0}
		<h1>{displayText}</h1>
		<div class="time">
			<h1>{timer}</h1>
		</div>
	{/if}
</div>

<style>
	.timer {
		text-align: center;
	}
	.time {
		font-size: 400%;
	}
</style>