pub extern crate macroquad;
use macroquad::prelude::*;
pub mod button;
pub mod column;
pub mod row;
pub mod text;

use auto_impl::auto_impl;

use self::row::Row;

pub trait App {
    fn tick(&mut self);
}

///Root of all other widgets. Represents the game window geometry.
pub struct View {
    ///Represents geometry of the window
    pub geometry: Geometry,
    ///Top most widget in the current view
    child: Box<dyn Widget>,
}

impl Default for View {
    fn default() -> Self {
        View {
            geometry: Geometry::new(Vector2::from(0, 0)),
            child: Box::new(Row::new()),
        }
    }
}

// High level abstraction representing a single UI view
impl View {
    #[cfg(feature = "debug_draw")]
    pub fn debug_draw(&self) {
        draw_circle(0f32, 0f32, 20f32, RED);
        draw_circle(screen_width(), 0f32, 20f32, RED);
        draw_circle(0f32, screen_height(), 20f32, RED);
        draw_circle(screen_width(), screen_height(), 20f32, RED);
    }
    pub fn draw(&self) {
        #[cfg(feature = "debug_draw")]
        self.debug_draw();
        self.child.draw();
    }

    pub fn tick(&mut self) {
        // redraw if any child in the widget tree requests a rebuild or window is resized
        if self.child.get_build() || self.resized() {
            self.build();
        }
        self.child.tick();
    }

    pub fn new<T: Widget + 'static>(child: T) -> View {
        View {
            geometry: Geometry {
                top_left: Vector2 { x: 0f32, y: 0f32 },
                top_left_curr: Vector2 { x: 0f32, y: 0f32 },
                sides: Vector2 { x: 0f32, y: 0f32 },
                abs_sides: Vector2 { x: 0f32, y: 0f32 },
                margins: Directions2D {
                    top: 0f32,
                    bottom: 0f32,
                    left: 0f32,
                    right: 0f32,
                },
                abs_margins: Directions2D {
                    top: 0f32,
                    bottom: 0f32,
                    left: 0f32,
                    right: 0f32,
                },
            },
            child: Box::new(child),
        }
    }

    pub fn build(&mut self) {
        self.geometry.abs_sides.x = screen_width();
        self.geometry.abs_sides.y = screen_height();
        self.geometry.top_left_curr = self.child.build(&self.geometry, None);
    }

    pub fn resized(&self) -> bool {
        if screen_height() == self.geometry.abs_sides.y
            && screen_width() == self.geometry.abs_sides.x
        {
            false
        } else {
            true
        }
    }
}

/// Represents a 2D vector value
#[derive(Copy, Clone)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn from(x: i32, y: i32) -> Self {
        Vector2 {
            x: x as f32,
            y: y as f32,
        }
    }
    pub fn new(x: f32, y: f32) -> Self {
        Vector2 { x, y }
    }
}

/// Represents the four 2D directions
#[derive(Copy, Clone)]
pub struct Directions2D {
    pub top: f32,
    pub bottom: f32,
    pub left: f32,
    pub right: f32,
}

impl Directions2D {
    pub fn new(top: f32, bottom: f32, left: f32, right: f32) -> Self {
        Directions2D {
            top,
            bottom,
            left,
            right,
        }
    }
}

///Describes rectangle geometry assigned to a widget
pub struct Geometry {
    //Only "public" data member
    ///length of sides in % of available space
    pub sides: Vector2,

    //Private stuff (not intended to be set by the user
    ///Coordinates of top left point
    pub top_left: Vector2,

    ///Coordinates of top left point from which to start drawing current widget
    pub top_left_curr: Vector2,

    ///absolute side length as set by the build algorithm
    pub abs_sides: Vector2,

    ///outer padding in widget, described vertically and horizontally, used optionally during draw. Represented as percentages.
    pub margins: Directions2D,

    ///the absolute margin dimensions, set by the build algorithm
    pub abs_margins: Directions2D,
}

impl Geometry {
    pub fn new(sides: Vector2) -> Self {
        if sides.x < 0f32 || sides.y < 0f32 {
            panic!("Widget has no geometry!");
        } else {
            Geometry {
                top_left: Vector2::new(0f32, 0f32),
                top_left_curr: Vector2::new(0f32, 0f32),
                abs_sides: Vector2::new(0f32, 0f32),
                sides,
                margins: Directions2D::new(
                    (100f32 - sides.y) / 2f32,
                    (100f32 - sides.y) / 2f32,
                    (100f32 - sides.x) / 2f32,
                    (100f32 - sides.x) / 2f32,
                ),
                abs_margins: Directions2D::new(
                    (100f32 - sides.y) / 2f32,
                    (100f32 - sides.y) / 2f32,
                    (100f32 - sides.x) / 2f32,
                    (100f32 - sides.x) / 2f32,
                ),
            }
        }
    }
}

#[auto_impl(&mut)]
pub trait Widget {
    fn draw(&self);

    /// Duties of build:
    /// 1. Set top left. The widget is drawn from the top left onwards.
    /// 2. Set abs size. The absolute size of the widget after accounting for the screen width by converting percentags
    /// 3. Set abs margins. Used for debug drawing. This is the calculated amount of margin from percentages or inherited from parent.
    /// 4. Call build on all children.
    /// 5. Return top left offset, ie, the top left starting point for the next widget to be drawn by the parent widget (it's next child).
    /// Assume the parent widget draws diagonally. This is so that the returing child widget is agnostic of what it's parent is (the parent may use horizontal, vertical, or both offsets.)
    fn build(&mut self, geometry: &Geometry, margin: Option<Directions2D>) -> Vector2;
    fn tick(&mut self);

    /// Get widget's absolute dimensions
    fn get_side(&self) -> Vector2;

    /// Get the widget's ID. This has to be set manually by the user using the id method of the widget,
    ///and can be used to uniquely identify the widget when communicating with external functions like in the button callbacks.
    fn get_id(&self) -> u16;

    /// A widget may need an explicit rebuild even when the window isn't resized because
    /// for example it just got a new child that needs building
    fn get_build(&self) -> bool;
}
