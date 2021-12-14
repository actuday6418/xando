#![allow(dead_code)]
use macroquad::prelude::*;

#[derive(Clone, PartialEq, Debug)]
///Stores possible states for each cell and for player that won
enum PlayerState {
    X,
    O,
    Unset,
}

///Stores coordinate and size of each button. X and Y are the top left coordinates of each button
struct Button {
    x: f32,
    y: f32,
    ///length of side of each button
    side: f32,
    color: Color,
    ///wether or not the button is being hovered
    is_hovered: bool,
}

struct Cell {
    button: Button,
    state: PlayerState,
}

struct GameState {
    cells: Vec<Cell>,
    ///Number of cells on one side of the square
    side_length: usize,
    ///Initially unset for no one won, and set to X or O when either wins
    pub won: PlayerState,
    ///stores old width and height to detect window resizes
    pub scr_width: f32,
    pub scr_height: f32,
    pub curr_player: PlayerState,
}

impl GameState {
    pub fn new(side_length: usize) -> Self {
        let width = screen_width();
        let height = screen_height();
        let mut cells: Vec<Cell> = Vec::new();
        //padding is two percent of screen height
        let padding: f32 = height * 2. / 100.;
        //represents length of side of each cell
        let side: f32 = (height - padding * (side_length as f32 + 1f32)) / (side_length as f32);
        //left coordinate from which to start placing buttons
        let l_offset: f32 = (width / 2.0) - (side + padding) * (side_length as f32) / 2f32;

        for index in 0..side_length * side_length {
            cells.push(Cell {
                button: Button {
                    x: l_offset + ((index % side_length) as f32) * (side + padding),
                    y: padding + ((index / side_length) as f32) * (side + padding),
                    side: side,
                    color: GREEN,
                    is_hovered: false,
                },
                state: PlayerState::Unset,
            });
        }

        GameState {
            side_length: side_length,
            cells: cells,
            curr_player: PlayerState::O,
            won: PlayerState::Unset,
            scr_height: height,
            scr_width: width,
        }
    }

    ///called when display resizes, basicly remaps buttons and other UI elements (in the future)
    pub fn remap(&mut self) {
        let width = screen_width();
        let height = screen_height();
        //padding is two percent of screen height
        let padding: f32 = height * 2. / 100.;
        //represents length of side of each cell
        let side: f32 =
            (height - padding * (self.side_length as f32 + 1f32)) / (self.side_length as f32);
        //left coordinate from which to start placing buttons
        let l_offset: f32 = (width / 2.0) - (side + padding) * (self.side_length as f32) / 2f32;
        for (index, mut cell) in self.cells.iter_mut().enumerate() {
            cell.button.x = l_offset + ((index % self.side_length) as f32) * (side + padding);
            cell.button.y = padding + ((index / self.side_length) as f32) * (side + padding);
            cell.button.side = side;
        }
    }

    ///draws all UI elements found in the game state
    pub fn draw(&self) {
        for cell in self.cells.iter() {
            draw_rectangle_lines(
                cell.button.x,
                cell.button.y,
                cell.button.side,
                cell.button.side,
                5.0,
                cell.button.color,
            );
        }
    }

    pub fn check_if_over(&mut self) -> bool {
        if self.check_column() || self.check_row() || self.check_diagonal() {
            return true;
        } else if self
            .cells
            .iter()
            .all(|cell| cell.state != PlayerState::Unset)
        {
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
                match self.cells[it2 - 1].state {
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
                match self.cells[it2 - 1].state {
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
            match self.cells[(it1 as usize) * self.side_length + it1 as usize].state {
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
                .state
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

    fn handle_input(&mut self) {
        //handle touch input first
        for touch in touches() {
            let mut toggle_player = false;
            match touch.phase {
                TouchPhase::Stationary | TouchPhase::Started | TouchPhase::Moved => {
                    let x = touch.position.x;
                    let y = touch.position.y;
                    //check if touch intersects any button
                    for mut cell in self.cells.iter_mut() {
                        if cell.button.x < x
                            && x < cell.button.x + cell.button.side
                            && cell.button.y < y
                            && y < cell.button.y + cell.button.side
                        {
                            if cell.button.color == GREEN {
                                cell.button.color = DARKGREEN;
                                cell.button.is_hovered = true;
                            }
                        } else {
                            cell.button.is_hovered = false;
                        }
                    }
                }
                TouchPhase::Ended => {
                    let x = touch.position.x;
                    let y = touch.position.y;

                    //check if touch intersects any button
                    for mut cell in self.cells.iter_mut() {
                        if cell.button.x < x
                            && x < cell.button.x + cell.button.side
                            && cell.button.y < y
                            && y < cell.button.y + cell.button.side
                            && cell.button.color != RED
                            && cell.button.color != YELLOW
                        {
                            cell.button.color = if self.curr_player == PlayerState::X {
                                RED
                            } else {
                                YELLOW
                            };

                            cell.state = self.curr_player.clone();
                            toggle_player = true;
                        }
                    }
                }
                _ => {}
            }
            if toggle_player {
                self.toggle_player();
            }
        }

        //handle mouse input
        let (x, y) = mouse_position();
        let mut toggle_player = false;
        for mut cell in self.cells.iter_mut() {
            //check if mouse is inside any buttons
            if cell.button.x < x
                && x < cell.button.x + cell.button.side
                && cell.button.y < y
                && y < cell.button.y + cell.button.side
            {
                if is_mouse_button_pressed(MouseButton::Left)
                    && cell.button.color != RED
                    && cell.button.color != YELLOW
                {
                    cell.button.color = if self.curr_player == PlayerState::X {
                        RED
                    } else {
                        YELLOW
                    };
                    cell.state = self.curr_player.clone();
                    toggle_player = true;
                } else if cell.button.color == GREEN {
                    cell.button.color = DARKGREEN;
                }
            } else if cell.button.color == DARKGREEN && !cell.button.is_hovered {
                cell.button.color = GREEN;
            }
        }
        if toggle_player {
            self.toggle_player();
        }
    }

    fn toggle_player(&mut self) {
        self.curr_player = match self.curr_player {
            PlayerState::X => PlayerState::O,
            _ => PlayerState::X,
        }
    }
}

///Check if window was resized
fn resized(state: &GameState) -> bool {
    if screen_height() == state.scr_height && screen_width() == state.scr_width {
        return false;
    } else {
        return true;
    }
}

#[macroquad::main("InputTouch")]
async fn main() {
    //restart when game ends
    loop {
        let mut state = GameState::new(3);
        //loop for frame drawing
        loop {
            clear_background(BLACK);

            if resized(&state) {
                state.remap();
            }

            if state.check_if_over() {
                break;
            }
            state.draw();
            state.handle_input();

            next_frame().await
        }
    }
}
