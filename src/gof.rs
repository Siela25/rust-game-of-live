use rand::Rng;
use coffee::graphics::{Frame, Mesh, Shape, Rectangle, Color};


pub struct GameOfLife{
    board: Vec<Vec<i32>>,
    generation: i64,
    board_size: usize
}

impl GameOfLife{
    pub fn new(board_size: usize) -> GameOfLife{
        GameOfLife{ board: vec![vec![0; board_size/2]; board_size/2], generation: 1, board_size: board_size/2 }
    }

    pub fn rand_board_state(mut self, chance: i32) -> Self{
        let mut rng = rand::thread_rng();
        for row in 0..self.board.len(){
            for mut col in 0..self.board[row].len(){
                self.board[row][col] = if rng.gen_range(0..chance+1) == chance { 1 } else { 0 };
            }
        }

        /*self.board[50][50] = 1;
        self.board[50][51] = 1;


        self.board[48][50] = 1;
        self.board[48][51] = 1;

        self.board[49][49] = 1;

        self.board[49][52] = 1;*/


        self
    }

    pub fn draw(&mut self, frame: &mut Frame<'_>){
        let mut mesh = Mesh::new();
        for row in 0..self.board.len(){
            for mut col in 0..self.board[row].len(){
                if self.board[row][col] == 1 {
                    mesh.fill(Shape::Rectangle(Rectangle {
                        x: row as f32 * 2.0,
                        y: col as f32 * 2.0,
                        width: 2.0,
                        height: 2.0,
                    }), Color::WHITE);
                }
            }
        }
        mesh.draw(&mut frame.as_target())
    }

    pub fn check_rules(&mut self){
        self.generation = self.generation + 1;
        for row in 0..self.board.len() {
            for mut col in 0..self.board[row].len() {
                let neighbors = self.count_neighbors(row,col);
                let live = self.board[row][col] == 1;
                if live && neighbors < 2{
                    self.board[row][col] = 0;
                }
                if live && (neighbors == 2 || neighbors == 3){
                    self.board[row][col] = 1;
                }
                if live && neighbors > 3{
                    self.board[row][col] = 0;
                }
                if !live && neighbors == 3{
                    self.board[row][col] = 1;
                }
            }
        }
    }

    fn count_neighbors(&self, row: usize, col: usize) -> usize{
        let mut neighbors: usize = 0;
        if col + 1 < self.board_size && self.board[row][col + 1] == 1{ // TOP
            neighbors += 1;
        }
        if col +1 < self.board_size && row + 1 < self.board_size && self.board[row +1][col +1] == 1{ // TOP RIGHT
            neighbors += 1;
        }
        if row + 1 < self.board_size && self.board[row +1][col] == 1{// RIGHT
            neighbors += 1;
        }
        if row + 1 < self.board_size && is_less_than_zero(col) && self.board[row + 1][col - 1] == 1{// BOTTOM RIGHT
            neighbors += 1;
        }
        if is_less_than_zero(col) && self.board[row][col - 1] == 1{ // BOTTOM
            neighbors += 1;
        }
        if is_less_than_zero(row) && is_less_than_zero(col) && self.board[row - 1][col - 1] == 1{// BOTTOM LEFT
            neighbors += 1;
        }
        if is_less_than_zero(row) && self.board[row - 1][col] == 1{ //LEFT
            neighbors += 1;
        }
        if is_less_than_zero(row) && col + 1 < self.board_size && self.board[row - 1][col + 1] == 1{ // TOP LEFT
            neighbors += 1;
        }

        neighbors
    }

    pub fn debug(self) -> Self{
        println!("{:?}", self.board);
        self
    }




}

fn is_less_than_zero(value: usize) -> bool{
    if value.checked_sub(1).is_some(){
        value - 1 > 0
    }else {
        false
    }
}