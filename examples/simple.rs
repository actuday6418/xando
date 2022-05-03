use mcgooey::{
    button::Button,
    column::Column,
    macroquad::{self, prelude::*},
    text::Text,
    Geometry, Vector2, View,
};
use std::cell::RefCell;
use std::rc::Rc;

fn main_menu_ui(state: Rc<RefCell<()>>) -> View {
    View::new(
        Column::new().push(
            Button::default(state.clone())
                .color(WHITE)
                .is_pressed_callback(|_button: &mut Button<()>| {
                    panic!("EXIT");
                })
                .child(
                    Text::default()
                        .text("Click me to exit")
                        .geometry(Geometry::new(Vector2::from(90, 90)))
                        .color(RED),
                ),
        ),
    )
}

#[macroquad::main("XandO")]
async fn main() {
    let mut main_menu_ui = main_menu_ui(Rc::new(RefCell::new(())));
    loop {
        main_menu_ui.tick();
        main_menu_ui.draw();

        next_frame().await
    }
}
