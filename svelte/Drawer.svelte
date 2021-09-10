<script>
	import { selectedKeyPeg, selectedCodePeg } from "./stores.js";

	let colorPegs = [
		{
			value: "none"
		},
		{
			value: "black"
		},
		{
			value: "white"
		},
		{
			value: "red"
		},
		{
			value: "yellow"
		},
		{
			value: "green"
		},
		{
			value: "blue"
		}
	];

	let keyPegs = [
		{
			value: "none"
		},
		{
			value: "red"
		},
		{
			value: "white"
		}
	];
</script>

<div>
	<section>
		<h1>Code Selection</h1>
		<ul>
			{#each colorPegs as {value}, index}
				<li>
					<input type="radio" id={`code-${value}`} value={index - 1} name="code-peg" bind:group={$selectedCodePeg}>
					<label for={`code-${value}`}>{value}</label>
				</li>
			{/each}
		</ul>
	</section>
	<section>
		<h1>Key Selection</h1>
		<ul>
			{#each keyPegs as {value}, index}
				<li>
					<input type="radio" id={`key-${value}`} value={index - 1} name="key-peg" bind:group={$selectedKeyPeg}>
					<label for={`key-${value}`}>{value}</label>
				</li>
			{/each}
		</ul>
	</section>
</div>

<style>
	div {
		display: flex;
		flex-direction: column;
		align-items: left;
		justify-content: space-evenly;
	}
	section {
		margin: 0 20px 0 20px;
	}
	ul {
		list-style: none;
		margin: 10px 0 0 10px;
	}
	li {
		display: block;
	}
	label {
		text-transform: capitalize;
	}
</style>