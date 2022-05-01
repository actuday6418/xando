/*
This file is a copy of row.rs, with appopriate changes made for redering vertically instead of horizontally. Changes not pertaining
to this difference must be made in row.rs as well.
*/

use super::{Directions2D, Geometry, Vector2, Widget};
use macroquad::prelude::*;

pub struct Column {
    children: Vec<Box<dyn Widget>>,
    geometry: Geometry,
    id: u16,
}

impl Column {
    pub fn new() -> Self {
        Column {
            children: Vec::new(),
            id: 0,
            geometry: Geometry::new(Vector2 {
                x: 100f32,
                y: 100f32,
            }),
        }
    }

    pub fn children(self, children: Vec<Box<dyn Widget>>) -> Self {
        Column { children, ..self }
    }

    pub fn geometry(self, geometry: Geometry) -> Self {
        Column { geometry, ..self }
    }

    pub fn id(self, id: u16) -> Self {
        Self { id, ..self }
    }

    #[cfg(feature = "debug_draw")]
    fn debug_draw(&self) {
        draw_rectangle_lines(
            self.geometry.top_left.x,
            self.geometry.top_left.y,
            self.geometry.abs_sides.x,
            self.geometry.abs_sides.y,
            1.0,
            MAGENTA,
        );
        draw_circle(
            self.geometry.top_left.x,
            self.geometry.top_left.y,
            20f32,
            RED,
        );
    }
}

impl Widget for Column {
    fn get_build(&self) -> bool {
        for child in &self.children {
            if child.get_build() {
                return true;
            }
        }
        false
    }

    fn get_id(&self) -> u16 {
        self.id
    }

    fn draw(&self) {
        #[cfg(feature = "debug_draw")]
        self.debug_draw();
        for child in self.children.iter() {
            child.draw();
        }
    }

    fn tick(&mut self) {
        for child in self.children.iter_mut() {
            child.tick();
        }
    }

    fn get_side(&self) -> Vector2 {
        self.geometry.sides
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
            x: geometry.top_left.x + margins.left,
            y: geometry.top_left.y + margins.top,
        };
        self.geometry.top_left = tl;
        self.geometry.top_left_curr = self.geometry.top_left;

        //calculate spacing for children and verify that there is no overflow

        //records space currently occupied by widget's children, used to calculate margins
        let mut occupied_space: f32 = 0f32;
        let count: i32 = self.children.len() as i32 + 1;
        for child in self.children.iter() {
            occupied_space += child.get_side().y;
        }
        if occupied_space > 100f32 {
            panic!("Overflow! widgets exceeded 100!");
        }
        let mut child_margins = Directions2D::new(
            (100f32 - occupied_space) * dimensions.y / 100f32,
            (100f32 - occupied_space) * dimensions.y / 100f32,
            0f32,
            0f32,
        );

        //divide total margin by count to get margin for each widget
        child_margins.top /= count as f32;
        child_margins.bottom /= count as f32;

        let mut it = self.children.iter_mut();

        if let Some(child) = it.next() {
            let h_margin = (100f32 - child.get_side().x) * dimensions.x / 200f32;

            self.geometry.top_left_curr.y = child
                .build(
                    &self.geometry,
                    Some(Directions2D {
                        left: h_margin,
                        right: h_margin,
                        ..child_margins
                    }),
                )
                .y;
        }
        while let Some(child) = it.next() {
            let h_margin = (100f32 - child.get_side().x) * dimensions.x / 200f32;

            self.geometry.top_left_curr.y = child
                .build(
                    &self.geometry,
                    Some(Directions2D {
                        top: 0f32,
                        left: h_margin,
                        right: h_margin,
                        ..child_margins
                    }),
                )
                .y;
        }
        //return TL offset
        Vector2 {
            x: geometry.top_left_curr.x + margins.left + dimensions.x + margins.right,
            y: geometry.top_left_curr.y + margins.top + dimensions.y + margins.bottom,
        }
    }
}
