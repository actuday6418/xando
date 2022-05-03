# McGooey
McGooey is a GUI system written in Rust on top of Macroquad.
McGooey is Mc — Macroquad, and, Gooey — GUI. Also GUIs made with McGooey are indeed gooey.

GUIs in McGooey are written as a tree of Widgets. `Widget` is a trait that may be implemented to create new Widgets for your UI.
McGooey is also very opinionated about how widgets are laid out. Each Widget is logically supposed to draw only within the space passed to it, which is that of it's parent widget, after allowing for margins and other children of that parent Widget. All Widgets therefore have their dimensions specified in percentages. Even text scales with the size of it's parent widget (as it's size is a percentage of that of it's parent).

A simple widget tree:

```rust
View::new(
        Column::new().push(
            Button::default(state.clone())
                .color(WHITE)
                .is_pressed_callback(|_button: &mut Button<()>| {       // Specify the type of the external state variable as a type parameter
                    panic!("EXIT");
                })
                .child(
                    Text::default()
                        .text("Click me to exit")
                        .geometry(Geometry::new(Vector2::from(90, 90))) // that is, occupy 90% of parent's width and height. 
                                                                        // 10% of vertical and horizontal padding are added automatically
                        .color(RED),
                ),
        ),
    )
```

A View is a single screen with a bunch of widgets defining what it looks like. Each View must pass it's interactive widgets an Rc<RefCell<T>> that is the external state that can be mutated by the widget as well as other components of your application. A function that returns a view constructed with the required external state (Unit type as an example):
  
```rust
  fn ui(state: Rc<RefCell<()>>) -> View {
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
    let mut ui = ui(Rc::new(RefCell::new(())));
    loop {
        ui.tick();
        ui.draw();

        next_frame().await
    }
}

```

# Xando 
Xando is a Tic Tac Toe implementation using McGooey, which showcases some common scenarios like mutating external state on events like a button click, etc. Xando, through McGooey, is able to easily implement a variable number of cells in the game.
        
## Xando running on Linux
        
![xando](https://user-images.githubusercontent.com/56124831/166473452-e04e5dfd-4a7b-4727-83cd-90523f668ea1.gif)

        
# Building
Building for the host platform is the usual `cargo build`

## For Android
1. Start by setting up a docker environment:
`docker pull notfl3/cargo-apk`
        
2. cd into your project's root (location of Cargo.toml) and to enter the Docker container's shell:
```
docker run --rm -v $(pwd):/root/src -w /root/src -it notfl3/cargo-apk /bin/bash
```
        
3. Run the following commands in the shell
`cargo update`
`cargo quad-apk build --release`
Your APKs should now be in `target/android-artifacts/release/apk`.
        
4. Look at [this](https://macroquad.rs/tutorials/android/) guide for help, and create an issue if there's trouble.
