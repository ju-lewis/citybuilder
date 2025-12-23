
mod world;
mod rendering;
mod game;
mod entities;


use game::Game;



fn main() {


    let mut game = Game::new();

    game.play();

}
