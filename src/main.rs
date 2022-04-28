mod game;
mod gui;

use game::GameState;
use gui::text::Text;
use gui::Geometry;
use gui::UIRoot;
use gui::Vector2;
use gui::{button::Button, column::Column, row::Row};
use macroquad::prelude::*;

#[derive(PartialEq)]
///Represents different UI sections
enum UIMode {
    MainMenu,
    Game,
}

fn main_menu_ui() -> Box<dyn gui::Widget> {
    Box::new(Column::new().children(vec![
            Box::new(
                Row::new()
                    .geometry(Geometry::new(Vector2::from(90, 50)))
                    .children(vec![
                        Box::new(
                            Button::default()
                                .geometry(Geometry::new(Vector2::from(40, 20)))
                                .color(WHITE)
                                .is_hovered_callback(|button: &mut Button| {
                                    button.set_color(BEIGE);
                                })
                                .is_not_hovered_callback(|button: &mut Button| {
                                    button.set_color(WHITE);
                                })
                                .is_pressed_callback(|button: &mut Button| {
                                    button.set_color(RED);
                                })
                                .child(Box::new(
                                    Text::default()
                                        .text(String::from(
                                            "delkmddkende kd kle",
                                        ))
                                        .geometry(Geometry::new(Vector2::from(90, 90)))
                                        .color(RED),
                                )),
                        ),
                        Box::new(
                            Button::default()
                                .geometry(Geometry::new(Vector2::from(40, 30)))
                                .color(BLUE)
                                .is_hovered_callback(|button: &mut Button| {
                                    button.set_color(BEIGE);
                                })
                                .is_not_hovered_callback(|button: &mut Button| {
                                    button.set_color(BLUE);
                                })
                                .is_pressed_callback(|button: &mut Button| {
                                    button.set_color(RED);
                                }),
                        ),
                    ]),
            ),
            Box::new(
                Button::default()
                    .geometry(Geometry::new(Vector2::from(30, 50)))
                    .color(ORANGE)
                    .is_hovered_callback(|button: &mut Button| {
                        button.set_color(BEIGE);
                    })
                    .is_not_hovered_callback(|button: &mut Button| {
                        button.set_color(ORANGE);
                    })
                    .is_pressed_callback(|button: &mut Button| {
                        button.set_color(RED);
                    }),
            ),
        ]))
}

#[macroquad::main("XandO")]
async fn main() {
    let mut mode = UIMode::MainMenu;
    let mut root = UIRoot::new(main_menu_ui());
    root.build();
    //restart when game ends
    loop {
        let mut state = GameState::new(3);
        //loop for frame drawing
        loop {
            clear_background(BLACK);

            if mode == UIMode::MainMenu {
                if root.resized() {
                    root.build();
                }
                root.tick();
                root.draw();
            } else {
                state.draw();
                state.handle_input();
            }
            next_frame().await
        }
    }
}
