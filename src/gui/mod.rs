use macroquad::prelude::*;

pub struct UIRoot {
    geometry: Geometry,
    child: Box<dyn Widget>,
}

impl UIRoot {
    pub fn draw(&self) {
        self.child.draw();
    }
    pub fn new(child: Box<dyn Widget>) -> UIRoot {
        UIRoot {
            geometry: Geometry {
                top_left: Vector2 { x: 0f32, y: 0f32 },
                top_left_curr: Vector2 { x: 0f32, y: 0f32 },
                sides: Vector2 { x: 0f32, y: 0f32 },
                abs_sides: Vector2 { x: 0f32, y: 0f32 },
                margins: Vector2 { x: 0f32, y: 0f32 },
            },
            child: child,
        }
    }

    pub fn remap(&mut self) {
        self.geometry.sides.x = screen_width();
        self.geometry.sides.y = screen_height();
        self.child.remap(&self.geometry);
    }
}

///Represents a 2D vector value
#[derive(Copy,Clone)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

///Describes rectangle geometry assigned to a widget
pub struct Geometry {
    ///Coordinates of top left point
    pub top_left: Vector2,
    ///Coordinates of top left point from which to start drawing current widget
    pub top_left_curr: Vector2,
    ///length of sides in % of avilable space
    pub sides: Vector2,
    ///absolute side length as set by the algorithm
    pub abs_sides: Vector2,
    ///outer padding in widget, described vertically and horizontally
    pub margins: Vector2,
}

impl Geometry {
    pub fn new(sides: Vector2) -> Self {
        if sides.x < 0f32 || sides.y < 0f32 {
            panic!("Widget has no geometry!");
        } else {
            Geometry {
                top_left: Vector2 { x: 0f32, y: 0f32 },
                top_left_curr: Vector2 { x: 0f32, y: 0f32 },
                abs_sides: Vector2 { x: 0f32, y: 0f32 },
                sides: sides,
                margins: Vector2 {
                    x: 100f32 - sides.x,
                    y: 100f32 - sides.y,
                },
            }
        }
    }
}

pub trait Widget {
    fn draw(&self);
    fn remap(&mut self, geometry: &Geometry) -> Vector2;
}

pub struct Row {
    children: Vec<Box<dyn Widget>>,
    geometry: Geometry,
}

impl Widget for Row {
    fn draw(&self) {
        for child in self.children.iter() {
            child.draw();
        }
    }

    fn remap(&mut self, geometry: &Geometry) -> Vector2 {
        //Find original dimensions of parent and
        //calculate dimensions of this widget from parent dimensions and positions from margins
        let dimensions = Vector2 {
            x: geometry.abs_sides.x * self.geometry.sides.x / 100f32,
            y: geometry.abs_sides.y * self.geometry.sides.y / 100f32,
        };
        self.geometry.abs_sides = dimensions;
        let margins = Vector2 {
            x: geometry.abs_sides.x * self.geometry.margins.x / 100f32,
            y: geometry.abs_sides.y * self.geometry.margins.y / 100f32,
        };
        let tl = Vector2 {
            x: geometry.top_left.x + margins.x,
            y: geometry.top_left.y + margins.y,
        };
        self.geometry.top_left = tl;
        for child in self.children.iter_mut() {
            self.geometry.top_left_curr = child.remap(&self.geometry);
        }
        //return TL offset
        Vector2 {
            x: margins.x + dimensions.x + margins.x,
            y: margins.y + dimensions.y + margins.y,
        }
    }
}

///Stores coordinate and size of each button. X and Y are the top left coordinates of each button
pub struct Button {
    geometry: Geometry,
    color: Color,
    ///wether or not the button is being hovered
    is_hovered: bool,
}

impl Button {
    pub fn new(geometry: Geometry, color: Color) -> Self {
        Button {
            geometry: geometry,
            color: color,
            is_hovered: false,
        }
    }
}

impl Widget for Button {
    fn draw(&self) {
        draw_rectangle_lines(
            self.geometry.top_left.x,
            self.geometry.top_left.y,
            self.geometry.sides.x,
            self.geometry.sides.y,
            5.0,
            self.color,
        );
    }

    fn remap(&mut self, geometry: &Geometry) -> Vector2 {
        //Find original dimensions of parent and
        //calculate dimensions of this widget from parent dimensions and positions from margins
        let dimensions = Vector2 {
            x: geometry.abs_sides.x * self.geometry.sides.x / 100f32,
            y: geometry.abs_sides.y * self.geometry.sides.y / 100f32,
        };
        self.geometry.abs_sides = dimensions;
        let margins = Vector2 {
            x: geometry.abs_sides.x * self.geometry.margins.x / 100f32,
            y: geometry.abs_sides.y * self.geometry.margins.y / 100f32,
        };
        let tl = Vector2 {
            x: geometry.top_left.x + margins.x,
            y: geometry.top_left.y + margins.y,
        };
        self.geometry.top_left = tl;
        //return TL offset
        Vector2 {
            x: margins.x + dimensions.x + margins.x,
            y: margins.y + dimensions.y + margins.y,
        }
    }
}
