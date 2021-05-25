use crate::app::App;
use coffee::graphics::WindowSettings;
use coffee::Game;
use crate::gof::GameOfLife;

mod app;
mod gof;



fn main() {
    println!("Hello");
   // let gof = GameOfLife::new(10,10).rand_board_state(10);

    App::run(WindowSettings{
        title: "Game of Life".to_string(),
        size: (app::WINDOW_SIZE, app::WINDOW_SIZE),
        resizable: false,
        fullscreen: false,
        maximized: false
    }).expect("Error when starting engine");

}
