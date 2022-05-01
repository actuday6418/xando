use mcgooey::macroquad::prelude::*;
use mcgooey::{
    button::Button, column::Column, row::Row, text::Text, Geometry, UIRoot, Vector2, Widget,
};
use std::{cell::RefCell, rc::Rc};

#[derive(Clone, PartialEq, Debug)]
///Stores possible states for each cell and for player that won
pub enum PlayerState {
    X,
    O,
    Unset,
}

impl Default for PlayerState {
    fn default() -> Self {
        PlayerState::Unset
    }
}

pub struct GameState {
    cells: Vec<PlayerState>,
    ///Number of cells on one side of the square
    side_length: usize,
    ///Initially unset for no one won, and set to X or O when either wins
    pub won: PlayerState,
    ///stores old width and height to detect window resizes
    pub curr_player: PlayerState,
}

impl GameState {
    pub fn new(side_length: usize) -> Self {
        GameState {
            side_length,
            cells: vec![PlayerState::default(); side_length * side_length],
            curr_player: PlayerState::O,
            won: PlayerState::Unset,
        }
    }

    /// Must be calle every frame. Return true to exit
    pub fn tick(&mut self) -> bool {
        if self.check_if_over() {
            println!("winner: {:?}", self.won);
            true
        } else {
            false
        }
    }

    pub fn check_if_over(&mut self) -> bool {
        if self.check_column() || self.check_row() || self.check_diagonal() {
            return true;
        } else if self.cells.iter().all(|state| *state != PlayerState::Unset) {
            return true;
        } else {
            return false;
        }
    }

    fn check_column(&mut self) -> bool {
        let mut it1: usize = 1;
        while it1 <= self.side_length {
            let mut x_present: bool = false;
            let mut o_present: bool = false;
            let mut none_present: bool = false;

            let mut it2: usize = it1;
            while it2 <= self.side_length * 2 + it1 {
                match self.cells[it2 - 1] {
                    PlayerState::X => x_present = true,
                    PlayerState::O => o_present = true,
                    PlayerState::Unset => none_present = true,
                }
                it2 += self.side_length;
            }

            if x_present && !o_present && !none_present {
                self.won = PlayerState::X;
                return true;
            } else if o_present && !x_present && !none_present {
                self.won = PlayerState::O;
                return true;
            }
            it1 += 1;
        }
        return false;
    }

    fn check_row(&mut self) -> bool {
        let mut it1: usize = 1;
        while it1 <= self.side_length * (self.side_length - 1) + 1 {
            let mut x_present: bool = false;
            let mut o_present: bool = false;
            let mut none_present: bool = false;

            let mut it2: usize = it1;
            while it2 <= it1 + self.side_length - 1 {
                match self.cells[it2 - 1] {
                    PlayerState::X => x_present = true,
                    PlayerState::O => o_present = true,
                    PlayerState::Unset => none_present = true,
                }
                it2 += 1;
            }

            if x_present && !o_present && !none_present {
                self.won = PlayerState::X;
                return true;
            } else if o_present && !x_present && !none_present {
                self.won = PlayerState::O;
                return true;
            }
            it1 += self.side_length;
        }
        return false;
    }

    fn check_diagonal(&mut self) -> bool {
        //check major diagonal first
        let mut it1: isize = 0;

        let mut x_present: bool = false;
        let mut o_present: bool = false;
        let mut none_present: bool = false;

        while it1 < self.side_length as isize {
            match self.cells[(it1 as usize) * self.side_length + it1 as usize] {
                PlayerState::X => x_present = true,
                PlayerState::O => o_present = true,
                PlayerState::Unset => none_present = true,
            }
            it1 += 1;
        }

        if x_present && !o_present && !none_present {
            self.won = PlayerState::X;
            return true;
        } else if o_present && !x_present && !none_present {
            self.won = PlayerState::O;
            return true;
        }

        x_present = false;
        o_present = false;
        none_present = false;
        it1 = (self.side_length - 1) as isize;

        //check minor diagonal
        while it1 >= 0 {
            match self.cells
                [it1 as usize + (self.side_length - it1 as usize - 1) * self.side_length]
            {
                PlayerState::X => x_present = true,
                PlayerState::O => o_present = true,
                PlayerState::Unset => none_present = true,
            }
            it1 -= 1;
        }

        if x_present && !o_present && !none_present {
            self.won = PlayerState::X;
            return true;
        } else if o_present && !x_present && !none_present {
            self.won = PlayerState::O;
            return true;
        } else {
            return false;
        }
    }

    fn toggle_player(&mut self) {
        self.curr_player = match self.curr_player {
            PlayerState::X => PlayerState::O,
            _ => PlayerState::X,
        }
    }
}

pub fn game_ui(state: Rc<RefCell<GameState>>) -> UIRoot {
    //represents each horizontal line of cells
    let mut rows: Vec<Box<dyn Widget>> = Vec::new();

    //calculate the precentage width and height to be given to each cell, as a part in the whole 100% of width or height
    let side_percentage = 100 / state.borrow().side_length;

    //add buttons to rows and rows to the column
    for i in 0..state.borrow().side_length {
        let mut children: Vec<Box<dyn Widget>> = Vec::new();
        for j in 0..state.borrow().side_length {
            children.push(Box::new(
                Button::default(state.clone())
                    .id((i * state.borrow().side_length + j) as u16)
                    .color(BEIGE)
                    .geometry(Geometry::new(Vector2::new(
                        (side_percentage - 10) as f32,
                        90f32,
                    )))
                    .is_hovered_callback(|button: &mut Button<GameState>| {
                        button.set_color(RED);
                    })
                    .is_not_hovered_callback(|button: &mut Button<GameState>| {
                        button.set_color(BEIGE);
                    })
                    .is_pressed_callback(|button: &mut Button<GameState>| {
                        button.set_child(Box::new(
                            Text::default()
                                .text((|| {
                                    {
                                        let mut state = button.state.borrow_mut();
                                        state.toggle_player();
                                        state.cells[button.id as usize] = state.curr_player.clone();
                                    }
                                    //use the opposite player because we just toggled
                                    if button.state.borrow().curr_player == PlayerState::X {
                                        "X"
                                    } else {
                                        "Y"
                                    }
                                })())
                                .geometry(Geometry::new(Vector2::from(80, 80))),
                        ));
                        button.set_build(true);
                    }),
            ));
        }
        let row = Row::new()
            .children(children)
            .geometry(Geometry::new(Vector2::new(100f32, side_percentage as f32)));
        rows.push(Box::new(row));
    }
    UIRoot::new(Box::new(Column::new().children(rows)))
}
