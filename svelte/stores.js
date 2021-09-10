import { writable } from "svelte/store";

import { Code, Key, Game } from "mastermind-wasm";

export const selectedKeyPeg = writable(-1);
export const selectedCodePeg = writable(-1);

let guesses = [];
let keys = [];

for (let i = 0; i < 12; i++) {
	guesses.push([-1, -1, -1, -1]);
	keys.push([-1, -1, -1, -1]);
}

let { subscribe, update, set } = writable({ guesses, keys });

export const boardState = {
	subscribe,
	update,
	set
};

({ subscribe, update, set } = writable(Game.new()));

let keyFromInput = (row) => {
	let white = 0, red = 0;
	for (let i = 0; i < row.length; i++) {
		if (row[i] == 0) red++;
		if (row[i] == 1) white++;
	}
	return Key.new(red, white);
};

export const gameState = {
	subscribe,
	update,
	set,
	reset: () => set(Game.new()),
	next: () => update(game => {
		let guesses_made = game.guesses_made;
		if (guesses_made > 0) {
			game.user_input(
				keyFromInput(keys[
					guesses_made - 1
				])
			);
		}
		game.next_guess();
		return game;
	}),
	back: () => update(game => {
		game.rollback(1);
		return game;
	})
};

gameState.subscribe(game => {
	let guesses = game.guesses();
	let board_guesses = [];

	game.print_num_possible_codes();

	for (let i = 0; i < 12; i++) {
		board_guesses.push([-1, -1, -1, -1]);
	}
	for (let i = 0; i < 12; i++) {
		if (guesses[i] !== null) {
			for (let j = 0; j < 4; j++) {
				board_guesses[i][j] = guesses[i][j];
			}
		}
	}

	boardState.update(board => {
		board.guesses = board_guesses;
		return board;
	});
});