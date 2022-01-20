mod gui;
mod maingame;
use crate::gui::Button;
use crate::gui::Geometry;
use crate::gui::UIRoot;
use crate::gui::Vector2;
use crate::maingame::GameState;
use macroquad::prelude::*;

#[derive(PartialEq)]
///Represents different UI sections
enum UIMode {
    MainMenu,
    Game,
}

///Check if window was resized
fn resized(state: &GameState) -> bool {
    if screen_height() == state.scr_height && screen_width() == state.scr_width {
        return false;
    } else {
        return true;
    }
}

#[macroquad::main("XandO")]
async fn main() {
    let mode = UIMode::MainMenu;
    let mut root = UIRoot::new(Box::new(Button::new(
        Geometry::new(Vector2 { x: 40f32, y: 40f32 }),
        WHITE,
    )));
    root.remap();
    //restart when game ends
    loop {
        let mut state = GameState::new(3);
        //loop for frame drawing
        loop {
            clear_background(BLACK);

            if mode == UIMode::MainMenu {
                if resized(&state) {
                    root.remap();
                }
                root.draw();
            } else {
                if resized(&state) {
                    state.remap();
                }

                if state.check_if_over() {
                    break;
                }
                state.draw();
                state.handle_input();
            }
            next_frame().await
        }
    }
}
