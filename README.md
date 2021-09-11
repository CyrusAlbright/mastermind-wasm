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

## Run locally

### Build

```
wasm-pack build
```

### Test

```
cargo test
```
