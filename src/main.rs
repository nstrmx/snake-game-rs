use game::*;
use raylib::prelude::*;
mod game;
// mod dqn;

const FPS: u8 = 120;
const SPEED: u8 = 16;

struct Controller {
    actions: Vec<Action>
}

impl Controller {
    fn track_action(&mut self, rl: &mut RaylibHandle) {
        if let Some(key) = rl.get_key_pressed() {
            if key == KeyboardKey::KEY_LEFT {
                self.actions.push(Action::LEFT);
            }
            else if rl.is_key_pressed(KeyboardKey::KEY_RIGHT) {
                self.actions.push(Action::RIGHT);
            }
        };
    }

    fn get_action(&mut self) -> Action {
        return if let Some(value) = self.actions.pop() {
            value
        } else {
            Action::FORWARD
        };
    }
}

fn main() {
    let mut game = Game::new();
    let mut controller = Controller{actions: vec![]};

    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
        .title("Snake game")
        .vsync()
        .build();
    rl.set_target_fps(FPS as u32);

    let mut frame: u8 = 0;
    while !rl.window_should_close() {
        if rl.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
            break;
        }
        
        controller.track_action(&mut rl);

        if frame > FPS / SPEED {
            frame = 0;

            let action = controller.get_action();
            let (state, reward, done) = game.step(action);

            if done {
                let state = game.reset();
            }
        }

        let renderer = rl.begin_drawing(&thread);
        game.render(renderer);

        frame += 1;
    }
}
