pub mod code;
pub mod game;
pub mod key;
pub mod prelude;

pub use prelude::*;

// until const generics are supported, these have to be constants
// if these are changed, several unit tests need to be changed to reflect
pub const NUM_COLORS: u8 = 6;
pub const NUM_SLOTS: u8 = 4;
pub const MAX_GUESSES: u32 = 12;

pub const NUM_CODES: u32 = (NUM_COLORS as u32).pow(NUM_SLOTS as u32);
pub const NUM_KEYS: u32 = (NUM_SLOTS as u32 + 1) * (NUM_SLOTS as u32 + 2) / 2;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    Ok(())
}