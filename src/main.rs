use crate::board_controller::BoardController;
use crate::board::{CoordinatePair, Board};

mod board_controller;
mod kilobot;
mod board;

pub const PI :f64 = std::f64::consts::PI;

fn main() {
    let mut new_bot = kilobot::new_kilobot(0);
    test_bot(&mut new_bot);
    let mut new_board = Board::new(5, 5);
    //test_board(&mut new_board);
    let mut new_board_controller = &mut BoardController{ board: new_board};
    test_board_controller(new_board_controller);
    //test_math();
}

fn test_math()
{
    let a :f64 = 0.0;
    let b :f64 = 270.0;
    println!("a: {}: dX = {}, dY = {}",a,a.to_radians().sin().round(),a.to_radians().cos().round());
    println!("b: {}: dX = {}, dY = {}",b,b.to_radians().sin().round(),b.to_radians().cos().round());

    let c = a.to_radians().cos().round() as i16;
    println!("1 == cos(0): {}", c == 1_i16);
}

fn test_bot(bot: &mut kilobot::Kilobot)
{
    println!("Bot: {}", bot);
    assert_eq!(bot.get_motor_values(), (0,0));
    bot.move_forward();
    assert_eq!(bot.get_motor_values(), (kilobot::MOTOR_MAX_VAL, kilobot::MOTOR_MAX_VAL));
}

fn test_board(board: &mut board::Board)
{
    board.add_new_bot_at_coord(kilobot::new_kilobot(1), CoordinatePair{x: 0, y: 0}, 0);
    board.add_new_bot_at_coord(kilobot::new_kilobot(2), CoordinatePair{x: 0, y: 0}, 0);
    board.add_new_bot_at_coord(kilobot::new_kilobot(3), CoordinatePair{x: 2, y: 2}, 0);
    board.remove_bot_location_at_coord(CoordinatePair{x: 0, y: 0},);
    println!("Board: {}", board);
    board.print_board();
    board.remove_bot_location_at_coord(CoordinatePair{x: 2, y: 2},);
    println!("Board: {}", board);
    board.print_board();
    board.add_new_bot_at_coord(kilobot::new_kilobot(1), CoordinatePair{x: 1, y: 1}, 0);
    println!("Board: {}", board);
    board.print_board();
}

fn test_board_controller(board_controller: &mut board_controller::BoardController)
{
    test_board(&mut board_controller.board);
    board_controller.move_bot_by_coord(CoordinatePair{x: 1, y: 1},CoordinatePair{x: 0, y: 0});
    println!("Board: {}", board_controller.board);
    board_controller.board.print_board();
    board_controller.board.add_new_bot_at_coord(kilobot::new_kilobot(3), CoordinatePair{x: 2, y: 2}, 180);
    board_controller.move_bot_by_index(0,4);
    board_controller.move_bot_by_coord(CoordinatePair{x: 2, y: 2},CoordinatePair{x: 3, y: 2});
    board_controller.move_bot_by_index(13,18);
    board_controller.move_bot_by_coord(CoordinatePair{x: 4, y: 0},CoordinatePair{x: 4, y: 1});
    println!("Board: {}", board_controller.board);
    board_controller.board.print_board();
    board_controller.board.add_new_bot_at_coord(kilobot::new_kilobot(4), CoordinatePair{x:1, y:1}, 135);
    board_controller.move_bot_forward(6);
    board_controller.move_bot_forward(18);
    println!("Board: {}", board_controller.board);
    board_controller.board.print_board();
    board_controller.move_bot_forward(23);
    println!("Board: {}", board_controller.board);
    board_controller.board.print_board();

}



