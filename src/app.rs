use coffee::{Game, Timer};
use coffee::graphics::{Window, Frame, Color, Image, Mesh, Shape, Rectangle, Point, Vector};
use coffee::input::{Input, Event};
use coffee::load::{Task, LoadingScreen};
use crate::gof::GameOfLife;

pub static WINDOW_SIZE: u32 = 400;

pub struct App{
    game_of_life: GameOfLife,
    cursor_position: Point
}

impl Game for App{
    type Input = ();
    type LoadingScreen = ();



    fn load(window: &Window) -> Task<Self> where
        Self: Sized {
        println!("LOAD!");

        Task::succeed(|| App{
            game_of_life: GameOfLife::new(WINDOW_SIZE as usize)
            .rand_board_state(50),
            cursor_position: Point::from([0.0, 0.0])
        })
    }




    fn draw(&mut self, frame: &mut Frame<'_>, timer: &Timer) {
        frame.clear(Color::BLACK);


        self.game_of_life.draw(frame);


        self.game_of_life.check_rules();





    }

}
