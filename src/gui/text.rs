use macroquad::prelude::*;

use super::{Directions2D, Geometry, Vector2, Widget};

pub struct Text {
    //offset the text by this much when rendering to center it within it's parent widget
    offset: Vector2,

    text: String,
    font_size: u16,
    geometry: Geometry,
    color: Color,
    resize_to_parent: bool,
}

impl Default for Text {
    fn default() -> Self {
        Text {
            offset: Vector2::from(0, 0),
            geometry: Geometry::new(Vector2::new(100f32, 100f32)),
            color: WHITE,
            resize_to_parent: true,
            text: String::new(),
            font_size: 10,
        }
    }
}

impl Text {
    pub fn geometry(self, geometry: Geometry) -> Self {
        Text { geometry, ..self }
    }

    pub fn color(self, color: Color) -> Self {
        Text { color, ..self }
    }

    pub fn resize_to_parent(self, resize_to_parent: bool) -> Self {
        Text {
            resize_to_parent,
            ..self
        }
    }

    pub fn text(self, text: String) -> Self {
        Text { text, ..self }
    }
}

impl Widget for Text {
    fn build(&mut self, geometry: &Geometry, margins: Option<Directions2D>) -> Vector2 {
        //Find original dimensions of parent and
        //calculate dimensions of this widget from parent dimensions and positions from margins
        let dimensions = Vector2 {
            x: geometry.abs_sides.x * self.geometry.sides.x / 100f32,
            y: geometry.abs_sides.y * self.geometry.sides.y / 100f32,
        };

        self.geometry.abs_sides = dimensions;

        //find the constraint for fitting text. Could be width or height.
        let text_dimensions = measure_text(self.text.as_str(), None, 1, 1f32);
        let height_ratio: f32 = dimensions.y / text_dimensions.height;
        let width_ratio: f32 = dimensions.x / text_dimensions.width;

        self.font_size = if width_ratio > height_ratio {
            //height is the constraining factor
            self.offset.y = 0f32;
            self.offset.x = self.geometry.abs_sides.x / 2f32 - text_dimensions.width / 2f32;
            height_ratio as u16
        } else {
            //width is
            self.offset.x = 0f32;
            self.offset.y = self.geometry.abs_sides.y / 2f32 - text_dimensions.height / 2f32;
            width_ratio as u16
        };
        println!("{} {}", self.offset.x, self.offset.y);

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

        Vector2 {
            x: geometry.top_left_curr.x + margins.left + dimensions.x + margins.right,
            y: geometry.top_left_curr.y + margins.top + dimensions.y + margins.bottom,
        }
    }

    fn tick(&mut self) {}

    fn draw(&self) {
        draw_text_ex(
            self.text.as_str(),
            //Text drawing from the bottom left instead of top left. Add offsets to center the text.
            //For other types of justification, change the offset calculation in the build function.
            self.geometry.top_left.x + self.offset.x,
            self.geometry.top_left.y - self.offset.y + self.geometry.abs_sides.y,
            TextParams {
                font_size: self.font_size,
                color: self.color,
                ..TextParams::default()
            },
        );
    }

    fn get_side(&self) -> Vector2 {
        self.geometry.sides
    }
}
