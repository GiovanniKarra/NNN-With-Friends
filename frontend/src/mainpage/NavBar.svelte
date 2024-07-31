<script>
	import NavButton from "./NavButton.svelte";

</script>

<div class="navbar">
	<NavButton name = "LOL"/>
	<NavButton name = "MDR"/>
	<NavButton name = "YOOOO" dropdown = {["hey", "there"]}/>
</div>

<style>
	.navbar {
		display: flex;
		flex-direction: row;
		
		background-color: lightgray;
		padding-top: 5%;
		margin: 0;
	}
</style>