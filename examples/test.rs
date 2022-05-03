use mcgooey::{
    button::Button,
    column::Column,
    macroquad::{self, prelude::*},
    text::Text,
    Geometry, Vector2, View,
};
use std::cell::RefCell;
use std::rc::Rc;

fn main_menu_ui(state: Rc<RefCell<State>>) -> View {
    View::new(
        Column::new()
            .push(
                Button::default(state.clone())
                    .geometry(Geometry::new(Vector2::from(40, 40)))
                    .color(WHITE)
                    .is_pressed_callback(|_button: &mut Button<State>| {
                        panic!("EXIT");
                    })
                    .is_hovered_callback(|button: &mut Button<State>| {
                        button.set_color(BEIGE);
                    })
                    .is_not_hovered_callback(|button: &mut Button<State>| {
                        button.set_color(WHITE);
                    })
                    .child(
                        Text::default()
                            .text("Click me to exit")
                            .geometry(Geometry::new(Vector2::from(90, 90)))
                            .color(RED),
                    ),
            )
            .push(
                Button::default(state)
                    .geometry(Geometry::new(Vector2::from(40, 40)))
                    .color(WHITE)
                    .is_pressed_callback(|button: &mut Button<State>| {
                        let temp = button.state.borrow().counter.wrapping_add(5);
                        button.state.borrow_mut().counter = temp;
                        println!("Did nothing {} times", button.state.borrow().counter);
                        button.set_child(Box::new(
                            Text::default()
                                .text(
                                    format!(
                                        "Clicked to do nothing {} times",
                                        button.state.borrow().counter
                                    )
                                    .as_str(),
                                )
                                .color(BLUE)
                                .geometry(Geometry::new(Vector2::from(90, 90))),
                        ));
                        button.set_build(true);
                    })
                    .is_hovered_callback(|button: &mut Button<State>| {
                        button.set_color(BEIGE);
                    })
                    .is_not_hovered_callback(|button: &mut Button<State>| {
                        button.set_color(WHITE);
                    })
                    .child(
                        Text::default()
                            .text("Click me to do nothing")
                            .geometry(Geometry::new(Vector2::from(90, 90)))
                            .color(RED),
                    ),
            ),
    )
}

struct State {
    counter: u8,
}
#[macroquad::main("XandO")]
async fn main() {
    let state = State { counter: 0 };
    let mut main_menu_ui = main_menu_ui(Rc::new(RefCell::new(state)));
    loop {
        main_menu_ui.tick();
        main_menu_ui.draw();

        next_frame().await
    }
}
