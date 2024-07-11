fn main() {
    let _ = pollster::block_on(game_of_life::start());
}
