use std::cell::RefCell;
use std::rc::Rc;
mod game;
use game::{game_ui, GameState};
use mcgooey::button::Button;
use mcgooey::column::Column;
use mcgooey::macroquad::{self, prelude::*};
use mcgooey::text::Text;
use mcgooey::{Geometry, UIRoot, Vector2};

#[derive(PartialEq, Clone, Copy)]
///Represents different UI sections
enum UIMode {
    MainMenu,
    Game,
}

struct State {
    pub mode: UIMode,
}

fn main_menu_ui(state: Rc<RefCell<State>>) -> UIRoot {
    UIRoot::new(Box::new(Column::new().children(vec![Box::new(
            Button::default(state)
                .geometry(Geometry::new(Vector2::from(90, 40)))
                .color(WHITE)
                .is_pressed_callback(|button: &mut Button<State>| {
                    button.state.borrow_mut().mode = UIMode::Game;
                })
                .is_hovered_callback(|button: &mut Button<State>| {
                    button.set_color(BEIGE);
                })
                .is_not_hovered_callback(|button: &mut Button<State>| {
                    button.set_color(WHITE);
                })
                .child(Box::new(
                    Text::default()
                        .text("Click here to start playing")
                        .geometry(Geometry::new(Vector2::from(90, 90)))
                        .color(RED),
                )),
        )])))
}

#[macroquad::main("XandO")]
async fn main() {
    loop {
        let main_menu_state = Rc::new(RefCell::new(State {
            mode: UIMode::MainMenu,
        }));

        let mut main_menu_ui = main_menu_ui(main_menu_state.clone());
        while main_menu_state.borrow().mode == UIMode::MainMenu {
            main_menu_ui.tick();
            main_menu_ui.draw();

            next_frame().await
        }

        let game_state = Rc::new(RefCell::new(GameState::new(3)));
        let mut game_ui = game_ui(game_state.clone());

        while !game_state.borrow_mut().tick() {
            game_ui.tick();
            game_ui.draw();

            next_frame().await
        }
    }
}
