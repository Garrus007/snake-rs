function main(rust) {
    console.info("Rust WASM loaded");
    rust.main();
    let game = rust.Game.new();

    document.addEventListener('keydown', (e) =>  game.key_down(e));
    setInterval(() => game.update(), 30)
}

const rust = import('../pkg');
rust
    .then(main)
    .catch(console.error)
