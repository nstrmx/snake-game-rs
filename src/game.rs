use std::collections::LinkedList;
use rand::prelude::*;
use raylib::prelude::*;

pub const WINDOW_WIDTH: usize = 800;
pub const WINDOW_HEIGHT: usize = 600;
const GRID_SIZE: u8 = 16;
const GRID_DIM: usize = 400;
const GRID_X: usize = WINDOW_WIDTH / 2 - GRID_DIM / 2;
const GRID_Y: usize = WINDOW_HEIGHT / 2 - GRID_DIM / 2;

const SCORE: usize = GRID_SIZE as usize * GRID_SIZE as usize;

#[derive(Debug, PartialEq, Eq)]
pub enum Action {
    FORWARD,
    LEFT,
    RIGHT,
}

#[derive(Clone, PartialEq, Eq)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

type State = [[i32; GRID_SIZE as usize]; GRID_SIZE as usize];

pub struct Game {
    score: usize,
    max_score: usize,
    snake: Snake,
    apple: Apple,
    stage: Stage,
    state: State,
}

impl Game {
    pub fn new() -> Game {
        let stage = Stage{
            w: GRID_SIZE, 
            h: GRID_SIZE
        };
        let snake = Snake::new();
        let apple = Apple::new(&snake);
        let state = [[0; GRID_SIZE as usize]; GRID_SIZE as usize];
        return Game {
            score: SCORE,
            max_score: SCORE,
            snake,
            apple,
            stage,
            state
        }
    }

    pub fn step(&mut self, action: Action) -> (State, isize, bool) {
        self.snake.dir = match action {
            Action::LEFT => {
                match self.snake.dir {
                    Direction::UP => Direction::LEFT,
                    Direction::DOWN => Direction::RIGHT,
                    Direction::LEFT => Direction::DOWN,
                    Direction::RIGHT => Direction::UP,
                }
            }
            Action::RIGHT => {
                match self.snake.dir {
                    Direction::UP => Direction::RIGHT,
                    Direction::DOWN => Direction::LEFT,
                    Direction::LEFT => Direction::UP,
                    Direction::RIGHT => Direction::DOWN,
                }
            }
            _ => self.snake.dir.clone()
        };

        // if action == Direction::UP && self.snake.dir != Direction::DOWN 
        // || action == Direction::DOWN && self.snake.dir != Direction::UP 
        // || action == Direction::LEFT && self.snake.dir != Direction::RIGHT 
        // || action == Direction::RIGHT && self.snake.dir != Direction::LEFT {
        //     self.snake.dir = action;
        // }

        let reward;
        let mut done = false;

        let snake_pos = self.snake.next_pos();

        if self.snake.body.len() >= (GRID_SIZE as usize * GRID_SIZE as usize) {
            reward = SCORE as isize;
            done = true;
        } else if self.stage.overlaps(&snake_pos) || self.snake.overlaps(&snake_pos) {
            reward = -1 * (self.score - SCORE) as isize;
            done = true;
        } else if self.apple.overlaps(&snake_pos) {
            self.snake.increase(snake_pos.clone());
            self.apple = Apple::new(&self.snake);
            self.score += SCORE;
            reward = SCORE as isize;
        } else {
            self.snake.r#move(snake_pos);
            self.score -= 1;
            reward = -1;
        }

        if self.score > self.max_score {
            self.max_score = self.score;
            println!("game: new max score = {}", self.score);
        }

        let state = self.update_state();

        return (state, reward, done);
    }

    fn update_state(&mut self) -> State {
        for pos in self.snake.body.iter() {
            self.state[pos.x as usize][pos.y as usize] = 1;
        }
        let pos = &self.apple.pos;
        self.state[pos.x as usize][pos.y as usize] = 2;
        return self.state.clone();
    }

    pub fn reset(&mut self) -> State {
        self.snake = Snake::new();
        self.apple = Apple::new(&self.snake);
        self.state = [[0; GRID_SIZE as usize]; GRID_SIZE as usize];
        self.score = SCORE;
        return self.state;
    }

    pub fn render(&self, mut renderer: RaylibDrawHandle) {
        renderer.clear_background(Color::BLACK);
        renderer.draw_text(
            format!(
                "size {}\nspeed {}\nsnake {}\nscore {}\nmax score {}", 
                GRID_SIZE, 8, self.snake.body.len(), self.score, self.max_score
            ).as_str(), 
            18, 18, 18, Color::WHITE
        );
        self.stage.render(&mut renderer);
        self.apple.render(&mut renderer);
        self.snake.render(&mut renderer);
    }
}

#[derive(Clone)]
struct Position {
    x: i8, y: i8
}

struct Snake {
    dir: Direction,
    body: LinkedList<Position>
}

impl Snake {
    fn new() -> Snake {
        let pos = Position {
            x: (rand::random::<u8>() % (GRID_SIZE / 2) + GRID_SIZE / 4) as i8,
            y: (rand::random::<u8>() % (GRID_SIZE / 2) + GRID_SIZE / 4) as i8
        };
        let mut body = LinkedList::new();
        body.push_back(pos);
        let mut snake = Snake {
            dir: Direction::UP,
            body,
        };
        let next_pos = Snake::next_pos(&snake);
        Snake::increase(&mut snake, next_pos);
        return snake;
    }

    fn increase(&mut self, pos: Position) {
        self.body.push_front(pos);
    }

    fn next_pos(&self) -> Position {
        let mut pos = Position{x: 0, y: 0};
        if let Some(head) = self.body.front() {
            match self.dir {
                Direction::UP => {
                    pos.x = head.x;
                    pos.y = head.y - 1;
                }
                Direction::DOWN => {
                    pos.x = head.x;
                    pos.y = head.y + 1;
                }
                Direction::LEFT => {
                    pos.x = head.x - 1;
                    pos.y = head.y;
                }
                Direction::RIGHT => {
                    pos.x = head.x + 1;
                    pos.y = head.y;
                }
            }
        }
        return pos;
    }

    fn r#move(&mut self, pos: Position) {
        let _ = self.body.pop_back();
        self.increase(pos);
    }

    fn overlaps(&self, pos: &Position) -> bool {
        for seg in self.body.iter() {
            if pos.x == seg.x && pos.y == seg.y {
                return true;
            }
        }
        return false;
    }

    fn render(&self, renderer: &mut RaylibDrawHandle) {
        let cell_size = GRID_DIM / GRID_SIZE as usize - 1;
        for pos in self.body.iter() {
            renderer.draw_rectangle(
                (GRID_X + pos.x as usize * cell_size) as i32, 
                (GRID_Y + pos.y as usize * cell_size) as i32, 
                cell_size as i32, 
                cell_size as i32, 
                Color::DARKGREEN,
            )
        }
    }
}

struct Apple {
    pos: Position,
}

impl Apple {
    fn new(snake: &Snake) -> Self {
        return Self {
            pos: Self::next_pos(snake, 0, 0, GRID_SIZE)
        };
    }

    fn next_pos(snake: &Snake, off_x: u8, off_y: u8, area_size: u8) -> Position {
        let pos = Position{
            x: (off_x + rand::random::<u8>() % area_size) as i8, 
            y: (off_y + rand::random::<u8>() % area_size) as i8
        };

        if snake.overlaps(&pos) {
            let new_area_size = area_size / 2;
            for i in 0..2 {
                for j in 0..2 {
                    let new_off_x = off_x + i * new_area_size;
                    let x_end = new_off_x + new_area_size;
                    let new_off_y = off_y + j * new_area_size;
                    let y_end = new_off_y + new_area_size;

                    if let Some(tail) = snake.body.back() {
                        if tail.x < x_end as i8 && tail.y < y_end as i8 {
                            return Self::next_pos(snake, new_off_x, new_off_y, new_area_size);
                        }
                    }
                }
            }
        }
        return pos;
    }

    fn overlaps(&self, pos: &Position) -> bool {
        return self.pos.x == pos.x && self.pos.y == pos.y;
    }

    fn render(&self, renderer: &mut RaylibDrawHandle) {
        let cell_size = GRID_DIM / GRID_SIZE as usize - 1;
        renderer.draw_rectangle(
            (GRID_X + self.pos.x as usize * cell_size) as i32, 
            (GRID_Y + self.pos.y as usize * cell_size) as i32, 
            cell_size as i32, 
            cell_size as i32, 
            Color::RED,
        )
    }
}

struct Stage {
    w: u8,
    h: u8,
}

impl Stage {
    fn overlaps(&self, pos: &Position) -> bool {
        return pos.x < 0 || pos.x >= self.w as i8 || pos.y < 0 || pos.y >= self.h as i8;
    }

    fn render(&self, renderer: &mut RaylibDrawHandle) {
        let cell_size = GRID_DIM / GRID_SIZE as usize - 1;
        renderer.draw_rectangle_lines(
            GRID_X as i32, 
            GRID_Y as i32, 
            (GRID_DIM - cell_size + cell_size / 3 - 1) as i32 , 
            (GRID_DIM - cell_size + cell_size / 3 - 1) as i32 ,
            Color::DIMGRAY,
        );
        // for i in 0..GRID_SIZE as usize {
        //     for j in 0..GRID_SIZE as usize {
        //         renderer.draw_rectangle(
        //             (GRID_X + i * cell_size) as i32, 
        //             (GRID_Y + j * cell_size) as i32, 
        //             cell_size as i32 - 1, 
        //             cell_size as i32 - 1,
        //             Color::BLACK,
        //         )
        //     }
        // }
    }
}