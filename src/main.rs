extern crate core;

use std::{thread};
use std::io;
use std::io::Read;
use std::io::Write;
use std::sync::mpsc::{channel, Receiver};
use std::thread::sleep;
use std::time::Duration;

use rand::Rng;

use terminal::*;

use crate::Move::*;
use crate::terminal::Color;
use crate::terminal::Color::*;

#[macro_use]
mod terminal;

#[derive(Debug)]
enum Move {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Copy, Clone)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Copy, Clone)]
struct Block {
    a: Coord,
    b: Coord,
    c: Coord,
    d: Coord,
    color: Color,
}

impl Block {
    pub fn turn(&mut self) {
        match self.color {
            RED => {
                if self.a.x != self.b.x {
                    //  BC
                    // AD
                    self.a.y -= 2;
                    self.b.x -= 1;
                    self.c.x -= 1;
                } else {
                    // A
                    // BC
                    //  D
                    let delta_x = if self.d.x == BOARD_WIDTH - 1 { -1i8 } else { 0 };
                    self.a.y += 2;
                    self.a.x = (self.a.x as i8 + delta_x) as usize;
                    self.b.x += (1 + delta_x) as usize;
                    self.c.x += (1 + delta_x) as usize;
                    self.d.x = (self.d.x as i8 + delta_x) as usize;
                }
            }
            GREEN => {
                if self.a.x != self.b.x {
                    // ABCD
                    let delta_y = if self.d.y == BOARD_HEIGHT - 1 { -1 } else { 0 };
                    self.a.x = self.c.x;
                    self.a.y = (self.a.y as i8 - 2 + delta_y) as usize;

                    self.b.x = self.c.x;
                    self.b.y = (self.b.y as i8 - 1 + delta_y) as usize;

                    self.c.y = (self.c.y as i8 + delta_y) as usize;

                    self.d.x = self.c.x;
                    self.d.y = (self.d.y as i8 + 1 + delta_y) as usize;
                } else {
                    let delta_x = if self.d.x == BOARD_WIDTH - 1 { -1 } else { (0.min(self.d.x as i8 - 2)).abs() };
                    self.a.x = (self.a.x as i8 - 2 + delta_x) as usize;
                    self.a.y = self.c.y;

                    self.b.x = (self.b.x as i8 - 1 + delta_x) as usize;
                    self.b.y = self.c.y;

                    self.c.x = (self.c.x as i8 + delta_x) as usize;

                    self.d.x = (self.d.x as i8 + 1 + delta_x) as usize;
                    self.d.y = self.c.y;
                }
            }
            YELLOW => {
                if self.a.y == self.d.y {
                    // BC
                    //  AD
                    self.a.y -= 2;
                    self.d.x -= 2;
                } else {
                    //  A
                    // BC
                    // D
                    let delta_x = if self.c.x == BOARD_WIDTH - 1 { -1i8 } else { 0 };
                    self.a.y += 2;
                    self.a.x = (self.a.x as i8 + delta_x) as usize;
                    self.b.x = (self.b.x as i8 + delta_x) as usize;
                    self.c.x = (self.c.x as i8 + delta_x) as usize;
                    self.d.x = (self.d.x as i8 + 2 + delta_x) as usize;
                }
            }
            MAGENTA => {
                if self.c.x + 1 == self.d.x {
                    // A
                    // B
                    // CD
                    let delta_x = if self.d.x == BOARD_WIDTH - 1 { -1i8 } else { 0 };
                    self.a.x = (self.a.x as i8 + delta_x) as usize;
                    self.a.y += 2;
                    self.b.x = (self.b.x as i8 + 1 + delta_x) as usize;
                    self.b.y += 1;
                    self.c.x = (self.c.x as i8 + 2 + delta_x) as usize;
                    self.d.x = (self.d.x as i8 + 1 + delta_x) as usize;
                    self.d.y -= 1;
                } else if self.d.y + 1 == self.a.y {
                    //
                    //   D
                    // ABC
                    self.a.x += 1;
                    self.b.y -= 1;
                    self.c.x -= 1;
                    self.c.y -= 2;
                    self.d.x -= 2;
                    self.d.y -= 1;
                } else if self.d.x + 1 == self.c.x {
                    // DC
                    //  B
                    //  A
                    let delta_x = if self.c.x == BOARD_WIDTH - 1 { -1i8 } else { 0 };
                    self.a.x = (self.a.x as i8 + 1 + delta_x) as usize;
                    self.a.y -= 1;
                    self.b.x = (self.b.x as i8 + delta_x) as usize;
                    self.c.x = (self.c.x as i8 - 1 + delta_x) as usize;
                    self.c.y += 1;
                    self.d.x = (self.d.x as i8 + delta_x) as usize;
                    self.d.y += 2;
                } else {
                    //
                    // CBA
                    // D
                    self.a.x -= 2;
                    self.a.y -= 1;
                    self.b.x -= 1;
                    self.c.y += 1;
                    self.d.x += 1;
                }
            }
            CYAN => {
                if self.a.x + 1 == self.d.x {
                    // AD
                    // B
                    // C
                    let delta_x = if self.d.x == BOARD_WIDTH - 1 { -1i8 } else { 0 };
                    self.a.x = (self.a.x as i8 + delta_x) as usize;
                    self.a.y += 2;
                    self.b.x = (self.b.x as i8 + 1 + delta_x) as usize;
                    self.b.y += 1;
                    self.c.x = (self.c.x as i8 + 2 + delta_x) as usize;
                    self.d.x = (self.d.x as i8 - 1 + delta_x) as usize;
                    self.d.y += 1;
                } else if self.a.y == self.d.y + 1 {
                    //
                    // D
                    // ABC
                    self.a.x += 1;
                    self.b.y -= 1;
                    self.c.x -= 1;
                    self.c.y -= 2;
                    self.d.y += 1;
                } else if self.a.x == self.d.x + 1 {
                    //  C
                    //  B
                    // DA
                    let delta_x = if self.c.x == BOARD_WIDTH - 1 { -1i8 } else { 0 };
                    self.a.x = (self.a.x as i8 + 1 + delta_x) as usize;
                    self.a.y -= 1;
                    self.b.x = (self.b.x as i8 + delta_x) as usize;
                    self.c.x = (self.c.x as i8 - 1 + delta_x) as usize;
                    self.c.y += 1;
                    self.d.x = (self.d.x as i8 + 2 + delta_x) as usize;
                } else {
                    //
                    // CBA
                    //   D
                    self.a.x -= 2;
                    self.a.y -= 1;
                    self.b.x -= 1;
                    self.c.y += 1;
                    self.d.x -= 1;
                    self.d.y -= 2;
                }
            }
            WHITE => {
                if self.d.x + 1 == self.a.x {
                    //  A
                    // DB
                    //  C
                    let delta_x = if self.a.x == BOARD_WIDTH - 1 { -1i8 } else { 0 };
                    self.a.x = (self.a.x as i8 - 1 + delta_x) as usize;
                    self.a.y += 1;
                    self.b.x = (self.b.x as i8 + delta_x) as usize;
                    self.c.x = (self.c.x as i8 + 1 + delta_x) as usize;
                    self.c.y -= 1;
                    self.d.x = (self.d.x as i8 + 1 + delta_x) as usize;
                    self.d.y += 1;
                } else if self.d.y == self.c.y + 1 {
                    //
                    // ABC
                    //  D
                    self.a.y -= 1;
                    self.b.x -= 1;
                    self.c.x -= 2;
                    self.c.y += 1;
                    self.d.y -= 1;
                } else if self.b.x + 1 == self.d.x {
                    // A
                    // BD
                    // C
                    let delta_x = if self.d.x == BOARD_WIDTH - 1 { -1i8 } else { 0 };
                    self.a.x = (self.a.x as i8 + delta_x) as usize;
                    self.a.y += 2;
                    self.b.x = (self.b.x as i8 + 1 + delta_x) as usize;
                    self.b.y += 1;
                    self.c.x = (self.c.x as i8 + 2 + delta_x) as usize;
                    self.d.x = (self.d.x as i8 + delta_x) as usize;
                } else {
                    //
                    //  D
                    // ABC
                    self.a.x += 1;
                    self.a.y -= 2;
                    self.b.y -= 1;
                    self.c.x -= 1;
                    self.d.x -= 1;
                }
            }
            _ => { /* BLUE square doesn't rotate */ }
        }
    }

    pub fn move_up(&mut self) {
        self.a.y -= 1;
        self.b.y -= 1;
        self.c.y -= 1;
        self.d.y -= 1;
    }

    pub fn move_down(&mut self) -> bool {
        if self.a.y > 18 || self.b.y > 18 || self.c.y > 18 || self.d.y > 18 {
            return false;
        }
        self.a.y += 1;
        self.b.y += 1;
        self.c.y += 1;
        self.d.y += 1;
        true
    }

    pub fn move_right(&mut self) -> bool {
        if self.a.x > 8 || self.b.x > 8 || self.c.x > 8 || self.d.x > 8 {
            return false;
        }
        self.a.x += 1;
        self.b.x += 1;
        self.c.x += 1;
        self.d.x += 1;
        true
    }

    pub fn move_left(&mut self) -> bool {
        if self.a.x < 1 || self.b.x < 1 || self.c.x < 1 || self.d.x < 1 {
            return false;
        }
        self.a.x -= 1;
        self.b.x -= 1;
        self.c.x -= 1;
        self.d.x -= 1;
        true
    }
}

// ms
const DRAW_SPEED: u64 = 50;

const SPEED: [u64; 11] = [
    1000 / DRAW_SPEED,
    800 / DRAW_SPEED,
    600 / DRAW_SPEED,
    500 / DRAW_SPEED,
    400 / DRAW_SPEED,
    350 / DRAW_SPEED,
    300 / DRAW_SPEED,
    250 / DRAW_SPEED,
    200 / DRAW_SPEED,
    150 / DRAW_SPEED,
    100 / DRAW_SPEED
];

const BLOCKS: [Block; 7] = [
    Block { a: Coord { x: 4, y: 0 }, b: Coord { x: 4, y: 1 }, c: Coord { x: 4, y: 2 }, d: Coord { x: 4, y: 3 }, color: GREEN },
    Block { a: Coord { x: 4, y: 0 }, b: Coord { x: 4, y: 1 }, c: Coord { x: 5, y: 0 }, d: Coord { x: 5, y: 1 }, color: BLUE },
    Block { a: Coord { x: 4, y: 0 }, b: Coord { x: 4, y: 1 }, c: Coord { x: 5, y: 1 }, d: Coord { x: 5, y: 2 }, color: RED },
    Block { a: Coord { x: 5, y: 0 }, b: Coord { x: 4, y: 1 }, c: Coord { x: 5, y: 1 }, d: Coord { x: 4, y: 2 }, color: YELLOW },
    Block { a: Coord { x: 4, y: 0 }, b: Coord { x: 4, y: 1 }, c: Coord { x: 4, y: 2 }, d: Coord { x: 5, y: 2 }, color: MAGENTA },
    Block { a: Coord { x: 4, y: 0 }, b: Coord { x: 4, y: 1 }, c: Coord { x: 4, y: 2 }, d: Coord { x: 5, y: 0 }, color: CYAN },
    Block { a: Coord { x: 5, y: 0 }, b: Coord { x: 5, y: 1 }, c: Coord { x: 5, y: 2 }, d: Coord { x: 4, y: 1 }, color: WHITE },
];

const BOARD_HEIGHT: usize = 20;
const BOARD_WIDTH: usize = 10;

struct Game {
    board: [[u8; BOARD_HEIGHT]; BOARD_WIDTH],
    block: Block,
    next_block: Block,
    lines_cleared: u32,
    level: usize,
    score: u32
}
impl Game {
    pub fn new() -> Game {
        Game {
            board: [[0u8; BOARD_HEIGHT]; BOARD_WIDTH],
            block: Self::random_block(),
            next_block: Self::random_block(),
            lines_cleared: 0,
            level: 0,
            score: 0
        }
    }

    pub fn clear_lines(&mut self) -> u32 {
        let mut lines_cleared = 0;
        for y in 0..BOARD_HEIGHT {
            if self.line_complete(y) {
                lines_cleared += 1;
                self.clear_line(y);
                self.move_lines_down(y);
                self.clear_line(0);
            }
        }
        self.lines_cleared += lines_cleared;
        return lines_cleared
    }

    pub fn add_lines_to_score(&mut self, lines: u32) {
        match lines {
            1 => self.score += 100,
            2 => self.score += 250,
            3 => self.score += 500,
            4 => self.score += 800,
            _ => {
                panic!("cleared {} lines?", lines);
            }
        }
    }

    fn line_complete(&self, line: usize) -> bool {
        for x in 0..BOARD_WIDTH {
            if self.board[x][line] == 0 {
                return false;
            }
        }
        true
    }

    fn move_lines_down(&mut self, start_above_line: usize) {
        for mut y2 in 0..start_above_line {
            y2 = start_above_line - 1 - y2;
            for x in 0..BOARD_WIDTH {
                self.board[x][y2 + 1] = self.board[x][y2];
            }
        }
    }

    fn clear_line(&mut self, line: usize) {
        for x in 0..BOARD_WIDTH {
            self.board[x][line] = 0;
        }
    }

    pub fn turn_block(&mut self) -> bool {
        self.block.turn();
        if !self.legal_move() {
            self.block.turn();
            self.block.turn();
            self.block.turn();
            return false;
        }
        true
    }

    pub fn move_block_down(&mut self) -> bool {
        if !self.block.move_down() {
            return false;
        } else if !self.legal_move() {
            self.block.move_up();
            return false;
        }
        true
    }

    pub fn move_block_right(&mut self) -> bool {
        if !self.block.move_right() {
            return false;
        } else if !self.legal_move() {
            self.block.move_left();
            return false;
        }
        true
    }

    pub fn move_block_left(&mut self) -> bool {
        if !self.block.move_left() {
            return false;
        } else if !self.legal_move() {
            self.block.move_right();
            return false;
        }
        true
    }

    fn random_block() -> Block {
        BLOCKS[rand::thread_rng().gen_range(0..7)]
    }

    fn legal_move(&self) -> bool {
        self.check_free(&self.block.a)
            && self.check_free(&self.block.b)
            && self.check_free(&self.block.c)
            && self.check_free(&self.block.d)
    }

    fn check_free(&self, coord: &Coord) -> bool {
        self.board[coord.x][coord.y] == 0
    }

    pub fn next_block(&mut self) -> bool {
        self.block = self.next_block;
        self.next_block = Self::random_block();
        if self.board[self.block.a.x][self.block.a.y] != 0
            || self.board[self.block.b.x][self.block.b.y] != 0
            || self.board[self.block.c.x][self.block.c.y] != 0
            || self.board[self.block.d.x][self.block.d.y] != 0 {
            return false;
        }
        true
    }

    pub fn cement_block(&mut self) {
        self.board[self.block.a.x][self.block.a.y] = self.block.color as u8;
        self.board[self.block.b.x][self.block.b.y] = self.block.color as u8;
        self.board[self.block.c.x][self.block.c.y] = self.block.color as u8;
        self.board[self.block.d.x][self.block.d.y] = self.block.color as u8;

        self.score += 10;

        let lines = self.clear_lines();
        if lines > 0 {
            self.add_lines_to_score(lines);

            if self.lines_cleared >= ((self.level + 1) * 10) as u32 {
                // upgrade level
                self.level = (self.level + 1).min(SPEED.len() - 1);
            }

            if lines == 4 {
                // celebrate tetris
                self.tetris();
            }
        }
    }

    pub fn draw(&self) {
        restore_cursor();
        overwrite();
        println!("{}", color!("   ┏━━━━━━━━━━┓", WHITE));
        for y in 0..BOARD_HEIGHT {
            overwrite();
            print!("{}", color!("   ┃", WHITE));
            for x in 0..BOARD_WIDTH {
                if Self::match_block(self.block, x, y) {
                    Self::draw_block(self.block.color as u8);
                } else {
                    if self.board[x][y] == 0 {
                        print!(" ");
                    } else {
                        Self::draw_block(self.board[x][y]);
                    }
                }
            }
            print!("{}", color!("┃ ", WHITE));
            match y {
                 4 => print!("{}", color!("Next", BLUE)),
                 5 => print!("{}", color!("Block", BLUE)),
                 7 => self.draw_block_line(0),
                 8 => self.draw_block_line(1),
                 9 => self.draw_block_line(2),
                10 => self.draw_block_line(3),
                12 => print!("{}", color!("Level", BLUE)),
                13 => print!("{0:5}", self.level + 1),
                14 => {}
                15 => print!("{}", color!("Lines", BLUE)),
                16 => print!("{0:5}", self.lines_cleared),
                17 => {}
                18 => print!("{}", color!("Score", BLUE)),
                19 => print!("{0:5}", self.score),
                _ => {}
            }
            println!();
        }
        overwrite();
        println!("{}", color!("   ┗━━━━━━━━━━┛", WHITE));
    }

    fn draw_block_line(&self, y: usize) {
        if Self::match_block(self.next_block, 4, y) {
            print!(" ");
            Self::draw_block(self.next_block.color as u8);
        } else {
            print!("  ");
        }
        if Self::match_block(self.next_block, 5, y) {
            Self::draw_block(self.next_block.color as u8);
        }
    }

    fn draw_block(color: u8) {
        print!("{}", inverted!("╳", color));
    }

    fn match_block(block: Block, x: usize, y: usize) -> bool {
        if block.a.x == x && block.a.y == y
            || block.b.x == x && block.b.y == y
            || block.c.x == x && block.c.y == y
            || block.d.x == x && block.d.y == y {
            return true
        }
        false
    }

    pub fn tetris(&self) {
        self.draw();
        restore_cursor();
        for _ in 0..10 {
            println!();
        }
        print!("{}", color!("   ┃", WHITE));
        println!("{}{}{}{}{}{}{}{}{}{}",
                 color!("~", WHITE),
                 color!("=", GREEN),
                 color!("T", YELLOW),
                 color!("E", RED),
                 color!("T", MAGENTA),
                 color!("R", BLUE),
                 color!("I", RED),
                 color!("S", YELLOW),
                 color!("=", GREEN),
                 color!("~", WHITE));
        sleep(Duration::from_millis(1500));
    }

    pub fn game_over(&self) {
        self.draw();
        restore_cursor();
        for y in 0..BOARD_HEIGHT + 2 {
            if y == 11 {
                print!("{}", color!("   ┃", WHITE));
                println!("{}", blink!("GAME OVER!", RED));
            } else {
                println!()
            }
        }
    }
}

fn main() {
    do_in_game_terminal(play_game);
}

fn play_game() {
    line_in_color("Welcome to Terminal Tetris!\n", YELLOW);
    create_game_screen(24);

    let mut game = Game::new();

    // input loop
    let (input_tx, input_rx) = channel();
    thread::spawn(move || {
        loop {
            let mov = move_by_player();
            input_tx.send(mov).unwrap();
        }
    });

    // ctrl-c trap
    let (ctrl_c_tx, ctrl_c_rx) = channel();
    ctrlc::set_handler(move || ctrl_c_tx.send(())
        .expect("Could not send signal on channel."))
        .expect("Error setting Ctrl-C handler");

    // game loop
    let mut game_over = false;
    for i in 0.. {
        let next_tick = i % SPEED[game.level] == 0;
        if next_tick {
            if !game.move_block_down() {
                game.cement_block();

                if !game.next_block() {
                    game_over = true;
                    break;
                }
            }
        }

        game.draw();

        handle_input(&mut game, &input_rx);

        if check_ctrl_c(&ctrl_c_rx) {
            // just stop game
            break;
        }

        sleep(Duration::from_millis(DRAW_SPEED));
    }

    if game_over {
        game.game_over();
    }
}

fn check_ctrl_c(rx: &Receiver<()>) -> bool {
    return match rx.try_recv() {
        Ok(_) => {
            true
        }
        Err(_) => {
            false
        }
    }
}

fn handle_input(game: &mut Game, rx: &Receiver<Move>) {
    loop {
        match rx.try_recv() {
            Ok(mov) => {
                match mov {
                    Up => game.turn_block(),
                    Down => game.move_block_down(),
                    Right => game.move_block_right(),
                    Left => game.move_block_left()
                };
            }
            Err(_) => {
                break;
            }
        }
    }
}

fn move_by_player() -> Move {
    overwrite();
    io::stdout().flush().expect("Cannot flush stdout");
    loop {
        for byte in io::stdin().bytes() {
            let input_key = byte.unwrap();

            match input_key {
                b'w' | b'A' => {
                    return Up;
                }
                b's' | b'B' => {
                    return Down;
                }
                b'd' | b'C' => {
                    return Right;
                }
                b'a' | b'D' => {
                    return Left;
                }
                _ => {}
            }
        }
    }
}