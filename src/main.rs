#![allow(dead_code)]

mod cube;
mod moves;
mod search;
mod display;

use std::env;

use cube::Cube;
use moves::Move;
use search::Search;

fn main() {
    let mut cube: Cube = Cube::new();

    // Getting arguments
    let _args: Vec<String> = env::args().collect();
    if _args.len() != 2 {
        println!("[!] Error, Rubik should receive one argument");
        return ;
    }

    // Making a list of Moves based on arguments
    let _moves: Vec<Move> = match Move::get_moves_list(&_args) {
        Some(value) => value,
        None => return
    };

    // [!] Debug : Display given Moves
    display::display_moves(&_moves);

    // Shuffle the cube
    Move::cube_shuffle(&mut cube, &_moves);

    // Search for a solution
    let _solution: Vec<Move> = Search::cube_solver(&mut cube);

    // Display solution
    display::display_solution(&_solution);
}

/* ---------------------------------------------------------------------------------
TESTS / RESEARCH / DRAFT :

[URF, UFL, ULB, UBR, DFR, DLF, DBL, DRB]
[0, 0, 0, 0, 0, 0, 0, 0]
[UR, UF, UL, UB, DR, DF, DL, DB, FR, FL, BL, BR]
[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]

        [T T T]
        [T T T]
        [T T T]

[L L L] [F F F] [R R R] [B B B]
[L L L] [F F F] [R R R] [B B B]
[L L L] [F F F] [R R R] [B B B]

        [D D D]
        [D D D]
        [D D D]

------
UP ROTATION :

[URF, UFL, ULB, UBR, DFR, DLF, DBL, DRB]
[0, 0, 0, 0, 0, 0, 0, 0]
[UR, UF, UL, UB, DR, DF, DL, DB, FR, FL, BL, BR]
[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]

        [T T T]
        [T T T]
        [T T T]

[F F F] [R R R] [B B B] [L L L]
[L L L] [F F F] [R R R] [B B B]
[L L L] [F F F] [R R R] [B B B]

        [D D D]
        [D D D]
        [D D D]

----------------------------------------------------------------------------------*/