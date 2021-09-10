<script>
	import { boardState, selectedKeyPeg } from "./stores.js";
</script>

<div class="container">
	{#each $boardState.guesses as row, i}
		<div class="row">
			<div class="code-row">
				<div class="background"></div>
				{#each row as peg}
					<span class="code-peg {peg >= 0 ? `active code-peg-${peg + 1}` : ""}"></span>
				{/each}
			</div>
			<div class="key-peg-container">
				{#each $boardState.keys[i] as peg, j}
					<button class="key-peg {peg >= 0 ? `key-peg-${peg + 1}` : ""}" on:click={() => {
						if ($boardState.keys[i][j] == $selectedKeyPeg) {
							$boardState.keys[i][j] = -1;
						} else {
							$boardState.keys[i][j] = $selectedKeyPeg;
						}
					}}></button>
				{/each}
			</div>
		</div>
	{/each}
</div>

<style>
	.container {
		/* flex-grow: 1; */

		height: 600px;
		overflow-y: auto;
		overflow-x: hidden;

		display: flex;
		flex-direction: column;
		align-items: center;
	}
	.row {
		display: flex;
		flex-direction: row;
		align-items: center;
	}
	.code-row {
		display: inline-block;
		position: relative;
		margin: 15px;

		display: flex;
		flex-direction: row;
	}
	.key-peg-container {
		display: inline-grid;
		grid-template-columns: 1fr 1fr;
		grid-template-rows: 1fr 1fr;
		gap: 2px;
		margin: 0 15px 0 0;
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

		width: 30px;
		height: 30px;
		border-radius: 15px;

		margin: 0 10px 0 10px;

		border: 2px solid var(--color-1);
		outline: 5px solid var(--background-accent);
		background-color: var(--background-color);
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
	.key-peg-1 {
		background-color: var(--color-3);
	}
	.key-peg-2 {
		background-color: var(--color-2);
	}
	button {
		border: 1px solid var(--color-1);
		width: 12px;
		height: 12px;
		border-radius: 10px;
		cursor: pointer;
		background: none;
		background-color: transparent;
		transition: background 100ms;
		appearance: none;
	}
	button:hover,
	button:focus {
		border-color: var(--background-accent);
	}
	button:focus {
		outline: 1px solid #fff;
		outline-offset: 1px;
	}
</style>