<div align="center">
	<h1><code>mastermind-wasm</code></h1>
	<strong>The classic puzzle game Mastermind, written in Rust WebAssembly and Svelte.</strong>
</div>

## About

Try it live on <a href="https://cyrusalbright.github.io">my github.io page</a>!

How to play:
- Come up with a code (and optionally mark it down at the bottom by selecting your colors and placing them in the slots)
- Press next to get the next guess
- Mark the amount of red and white pegs next to the computer's guess
- Repeat!

It's guaranteed to guess your code within 5 guesses.

### Why WebAssembly?

As a start, I implemented this algorithm with a naive approach in JavaScript and it was far too slow to be playable, taking several seconds between guesses.

I really enjoy using Rust, and since it has built-in support for compiling to WebAssembly and a mature ecosystem of libraries and utilities designed for WebAssembly, I figured I would implement the performance-sensitive parts in Rust.

## Run locally

### Build the WASM package

```
wasm-pack build
```

### Test the algorithm

```
cargo test
```

### Webpack test server

```
cd webpack
npm install
npm run dev
```