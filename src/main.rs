use std::collections::LinkedList;
use rand::prelude::*;
use raylib::prelude::*;

const WINDOW_WIDTH: usize = 800;
const WINDOW_HEIGHT: usize = 600;
const GRID_SIZE: u8 = 16;
const GRID_DIM: usize = 400;
const GRID_X: usize = WINDOW_WIDTH / 2 - GRID_DIM / 2;
const GRID_Y: usize = WINDOW_HEIGHT / 2 - GRID_DIM / 2;

const SCORE: usize = GRID_SIZE as usize * GRID_SIZE as usize;

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

struct GameState {
    score: usize,
    max_score: usize,
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

fn main() {
    let mut game = GameState {
        score: SCORE,
        max_score: SCORE,
    };
    let stage = Stage{
        w: GRID_SIZE, 
        h: GRID_SIZE
    };
    let mut snake = Snake::new();
    let mut apple = Apple::new(&snake);

    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
        .title("Snake game")
        .vsync()
        .build();
    rl.set_target_fps(8);

    while !rl.window_should_close() {
        if rl.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
            break;
        }
        if rl.is_key_down(KeyboardKey::KEY_UP) {
            match snake.dir {
                Direction::DOWN => (),
                _ => {snake.dir = Direction::UP}
            }
        }
        if rl.is_key_down(KeyboardKey::KEY_DOWN) {
            match snake.dir {
                Direction::UP => (),
                _ => {snake.dir = Direction::DOWN}
            }
        }
        if rl.is_key_down(KeyboardKey::KEY_LEFT) {
            match snake.dir {
                Direction::RIGHT => (),
                _ => {snake.dir = Direction::LEFT}
            }
        }
        if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            match snake.dir {
                Direction::LEFT => (),
                _ => {snake.dir = Direction::RIGHT}
            }
        }
         
        let snake_pos = snake.next_pos();

        if stage.overlaps(&snake_pos) || snake.overlaps(&snake_pos) {
            snake = Snake::new();
            game.score = SCORE;
        } else {
            if apple.overlaps(&snake_pos) {
                snake.increase(snake_pos.clone());
                apple = Apple::new(&snake);
                game.score += SCORE;
            } else {
                snake.r#move(snake_pos);
                game.score -= 1;
            }
        }

        if game.score > game.max_score {
            game.max_score = game.score;
            println!("game: new max score = {}", game.score);
        }

        let mut renderer = rl.begin_drawing(&thread);
        renderer.clear_background(Color::BLACK);
        renderer.draw_text(
            format!(
                "size {}\nspeed {}\nscore {}\nmax score {}", 
                GRID_SIZE, 8, game.score, game.max_score
            ).as_str(), 
            18, 18, 18, Color::WHITE
        );
        stage.render(&mut renderer);
        apple.render(&mut renderer);
        snake.render(&mut renderer);
    }
}
