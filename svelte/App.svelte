<script>
	import Drawer from "./Drawer.svelte";
	import Board from "./Board.svelte";

	import { gameState, selectedCodePeg } from "./stores.js";

	let userCode = [-1, -1, -1, -1];

</script>

<main>
	<div class="container stretch">
		<Board />
		<Drawer />
	</div>
	<div class="container center">
		<div class="code-row">
			<div class="background"></div>
			{#each userCode as peg, index}
				<button
					class="code-peg {peg >= 0 ? `active code-peg-${peg + 1}` : ""}"
					on:click={() => {
						userCode[index] = $selectedCodePeg;
					}}>
				</button>
			{/each}
		</div>
		<button on:click={() => {
			gameState.next();
		}}>Next guess</button>
		<button on:click={() => {
			gameState.back();
		}}>Back</button>
	</div>
</main>

<style>
	main {
		margin: 0 100px 0 100px;

		display: flex;
		flex-direction: column;
		align-items: center;
	}
	.container {
		display: flex;
		flex-direction: row;
		justify-content: center;
	}
	.stretch {
		align-items: stretch;
	}
	.center {
		align-items: center;
	}
	.code-row {
		display: inline-block;
		position: relative;
		margin: 15px 0;

		display: flex;
		flex-direction: row;
	}
	.background {
		position: absolute;
		left: 0;
		width: 100%;
		height: 70%;
		top: 50%;
		border-radius: 20px;
		transform: translateY(-50%);
		background-color: var(--background-accent);
		z-index: -1;
	}
	.code-peg {
		box-sizing: content-box;
		display: inline-block;

		position: relative;
		background: none;
		appearance: none;

		width: 30px;
		height: 30px;
		border-radius: 15px;

		margin: 0 10px 0 10px;
		
		border: 2px solid var(--color-1);
		outline: 5px solid var(--background-accent);
		background-color: var(--background-color);
		
		cursor: pointer;
	}
	.code-peg:focus {
		outline-color: white;
		outline-offset: 1px;
	}
	.code-peg:first-of-type {
		margin-left: 20px;
	}
	.code-peg:last-of-type {
		margin-right: 20px;
	}
	.code-peg.active::after {
		content: "";
		position: absolute;
		top: 30%;
		left: 70%;
		width: 1px;
		height: 1px;
		border-radius: 1px;

		transform: translate(-50%, -50%);

		background-color: rgba(255, 255, 255, 0.8);

		box-shadow: 0 0 2px 2px rgba(255, 255, 255, 0.8);
	}
	.code-peg-1 {
		background-color: var(--color-1);
	}
	.code-peg-2 {
		background-color: var(--color-2);
	}
	.code-peg-3 {
		background-color: var(--color-3);
	}
	.code-peg-4 {
		background-color: var(--color-4);
	}
	.code-peg-5 {
		background-color: var(--color-5);
	}
	.code-peg-6 {
		background-color: var(--color-6);
	}
</style>