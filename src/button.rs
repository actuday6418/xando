use super::{Directions2D, Geometry, Vector2, Widget};
use macroquad::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
///Stores coordinate and size of each button. X and Y are the top left coordinates of each button
pub struct Button<T> {
    //set by user
    geometry: Geometry,
    ///default color of button
    color: Color,
    //optional child widget
    child: Option<Box<dyn Widget>>,
    //called when mouse/finger enters hover over a button
    is_hovered_callback: fn(&mut Button<T>),
    //called when mouse/finger leaves hover over the button
    is_not_hovered_callback: fn(&mut Button<T>),
    is_pressed_callback: fn(&mut Button<T>),
    is_disabled: bool,

    ///should the widget be rebuilt?
    build: bool,

    ///wether or not the button is being hovered
    is_hovered: bool,
    pub id: u16,
    pub state: Rc<RefCell<T>>,
}

impl<T> Button<T> {
    pub fn default(state: Rc<RefCell<T>>) -> Self {
        Button {
            geometry: Geometry::new(Vector2::new(100f32, 100f32)),
            color: WHITE,
            child: None,
            is_hovered_callback: |_: &mut Button<T>| {},
            is_not_hovered_callback: |_: &mut Button<T>| {},
            is_pressed_callback: |_: &mut Button<T>| {},
            is_disabled: false,
            is_hovered: false,
            id: 0,
            state,
            build: false,
        }
    }

    pub fn set_build(&mut self, build: bool) {
        self.build = build;
    }

    pub fn geometry(self, geometry: Geometry) -> Self {
        Button { geometry, ..self }
    }

    pub fn color(self, color: Color) -> Self {
        Button { color, ..self }
    }

    pub fn is_hovered_callback(self, is_hovered_callback: fn(&mut Button<T>)) -> Self {
        Button {
            is_hovered_callback,
            ..self
        }
    }

    pub fn is_not_hovered_callback(self, is_not_hovered_callback: fn(&mut Button<T>)) -> Self {
        Button {
            is_not_hovered_callback,
            ..self
        }
    }

    pub fn is_pressed_callback(self, is_pressed_callback: fn(&mut Button<T>)) -> Self {
        Button {
            is_pressed_callback,
            ..self
        }
    }

    pub fn child<T2: Widget + 'static>(self, child: T2) -> Button<T> {
        Button {
            child: Some(Box::new(child)),
            ..self
        }
    }

    pub fn id(self, id: u16) -> Self {
        Self { id, ..self }
    }

    fn handle_input(&mut self) {
        //handle touch input first
        for touch in touches() {
            let x = touch.position.x;
            let y = touch.position.y;
            match touch.phase {
                TouchPhase::Stationary | TouchPhase::Started | TouchPhase::Moved => {
                    //check if touch intersects the button, call callbacks if user enters or leaves the hover state
                    if self.geometry.top_left.x < x
                        && x < self.geometry.top_left.x + self.geometry.abs_sides.x
                        && self.geometry.top_left.y < y
                        && y < self.geometry.top_left.y + self.geometry.abs_sides.y
                    {
                        if !self.is_hovered {
                            (self.is_hovered_callback)(self);
                            self.is_hovered = true;
                        }
                    } else {
                        if self.is_hovered {
                            (self.is_not_hovered_callback);
                            self.is_hovered = false;
                        }
                    }
                }
                TouchPhase::Ended => {
                    //check if touch intersects any button, call touched callback if user touches
                    if self.geometry.top_left.x < x
                        && x < self.geometry.top_left.x + self.geometry.abs_sides.x
                        && self.geometry.top_left.y < y
                        && y < self.geometry.top_left.y + self.geometry.abs_sides.y
                    {
                        if !self.is_disabled {
                            (self.is_pressed_callback)(self);
                        }
                    }
                }
                _ => {}
            }
        }

        //handle mouse input
        let (x, y) = mouse_position();
        //check if mouse is inside any button
        if self.geometry.top_left.x < x
            && x < self.geometry.top_left.x + self.geometry.abs_sides.x
            && self.geometry.top_left.y < y
            && y < self.geometry.top_left.y + self.geometry.abs_sides.y
        {
            if is_mouse_button_pressed(MouseButton::Left) && !self.is_disabled {
                (self.is_pressed_callback)(self);
            } else {
                if !self.is_hovered {
                    (self.is_hovered_callback)(self);
                    self.is_hovered = true;
                }
            }
        } else if self.is_hovered {
            (self.is_not_hovered_callback)(self);
            self.is_hovered = false;
        }
    }

    #[cfg(feature = "debug_draw")]
    fn debug_draw(&self) {
        draw_rectangle_lines(
            self.geometry.top_left.x - self.geometry.abs_margins.left,
            self.geometry.top_left.y - self.geometry.abs_margins.top,
            self.geometry.abs_sides.x
                + self.geometry.abs_margins.left
                + self.geometry.abs_margins.right,
            self.geometry.abs_sides.y
                + self.geometry.abs_margins.top
                + self.geometry.abs_margins.bottom,
            1.0,
            MAGENTA,
        );
        draw_circle(
            self.geometry.top_left.x,
            self.geometry.top_left.y,
            5f32,
            RED,
        );
    }
}

impl<T> Widget for Button<T> {
    fn draw(&self) {
        #[cfg(feature = "debug_draw")]
        self.debug_draw();
        draw_rectangle(
            self.geometry.top_left.x,
            self.geometry.top_left.y,
            self.geometry.abs_sides.x,
            self.geometry.abs_sides.y,
            self.color,
        );
        if let Some(widget) = &self.child {
            widget.draw();
        }
    }
    fn build(&mut self, geometry: &Geometry, margins: Option<Directions2D>) -> Vector2 {
        //Find original dimensions of parent and
        //calculate dimensions of this widget from parent dimensions and positions from margins
        let dimensions = Vector2 {
            x: geometry.abs_sides.x * self.geometry.sides.x / 100f32,
            y: geometry.abs_sides.y * self.geometry.sides.y / 100f32,
        };
        self.geometry.abs_sides = dimensions;
        let margins = margins.unwrap_or(Directions2D {
            top: geometry.abs_sides.y * self.geometry.margins.top / 100f32,
            bottom: geometry.abs_sides.y * self.geometry.margins.bottom / 100f32,
            left: geometry.abs_sides.x * self.geometry.margins.left / 100f32,
            right: geometry.abs_sides.x * self.geometry.margins.right / 100f32,
        });
        self.geometry.abs_margins = margins;

        let tl = Vector2 {
            x: geometry.top_left_curr.x + margins.left,
            y: geometry.top_left_curr.y + margins.top,
        };

        self.geometry.top_left = tl;
        self.geometry.top_left_curr = self.geometry.top_left;

        if let Some(widget) = &mut self.child {
            widget.build(&self.geometry, None);
        }
        //return TL offsetted
        Vector2 {
            x: geometry.top_left_curr.x + margins.left + dimensions.x + margins.right,
            y: geometry.top_left_curr.y + margins.top + dimensions.y + margins.bottom,
        }
    }

    fn tick(&mut self) {
        self.handle_input();
    }

    fn get_side(&self) -> Vector2 {
        self.geometry.sides
    }

    fn get_id(&self) -> u16 {
        self.id
    }

    fn get_build(&self) -> bool {
        if self.build {
            true
        } else if self.child.is_some() {
            self.child.as_ref().unwrap().get_build()
        } else {
            false
        }
    }
}

//Setters and getters for Button
impl<T> Button<T> {
    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn set_child(&mut self, child: Box<dyn Widget>) {
        self.child = Some(child);
    }

    pub fn set_is_disabled(&mut self, is_disabled: bool) {
        self.is_disabled = is_disabled;
    }

    pub fn set_geometry(&mut self, geometry: Geometry) {
        self.geometry = geometry;
    }

    pub fn set_is_hovered_callback(&mut self, is_hovered_callback: fn(&mut Button<T>)) {
        self.is_hovered_callback = is_hovered_callback;
    }

    pub fn set_is_not_hovered_callback(&mut self, is_not_hovered_callback: fn(&mut Button<T>)) {
        self.is_not_hovered_callback = is_not_hovered_callback;
    }

    pub fn set_is_pressed_callback(&mut self, is_pressed_callback: fn(&mut Button<T>)) {
        self.is_pressed_callback = is_pressed_callback;
    }

    pub fn get_color(&self) -> Color {
        self.color
    }

    pub fn get_is_disabled(&self) -> bool {
        self.is_disabled
    }
}
