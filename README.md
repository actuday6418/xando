# McGooey
McGooey is a GUI system written in Rust on top of Macroquad.
McGooey is Mc — Macroquad, and, Gooey — GUI. Also GUIs made with McGooey are indeed gooey.

GUIs in McGooey are written as a tree of Widgets. `Widget` is a trait that may be implemented to create new Widgets for your UI.
McGooey is also very opinionated about how widgets are laid out. Each Widget is logically supposed to draw only within the space passed to it, which is that of it's parent widget, after allowing for margins and other children of that parent Widget. All Widgets therefore have their dimensions specified in percentages. Even text scales with the size of it's parent widget (as it's size is a percentage of that of it's parent).

# Xando 
Xando is a Tic Tac Toe implementation using McGooey, which showcases some common scenarios like mutating external state on events like a button click, etc. Xando, through McGooey, is able to easily implement a variable number of cells in the game.
