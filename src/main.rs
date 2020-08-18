use crate::board_controller::BoardController;
use crate::board::{Board, bot_map, signal_map, CoordinatePair};
use crate::board::signal_map::SignalSource;

mod board_controller;
mod kilobot;
mod board;

pub const PI :f64 = std::f64::consts::PI;

fn main() {
    let mut new_bot = kilobot::new_kilobot(0);
    test_bot(&mut new_bot);
    //let new_board = Board::new(5, 5);
    //test_board(&mut new_board);
    //let new_board_controller = &mut BoardController{ board: new_board};
    //test_board_controller(new_board_controller);
    //test_math();
    let mut sig_map = signal_map::SignalMap::new(5,5);
    test_signal_map(&mut sig_map);

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

fn test_bot_map(bot_map: &mut bot_map::BotMap)
{
    bot_map.add_new_bot_at_index(kilobot::new_kilobot(1), 0, 0);
    bot_map.add_new_bot_at_index(kilobot::new_kilobot(2), 0, 0);
    bot_map.add_new_bot_at_index(kilobot::new_kilobot(3), 12, 0);
    bot_map.remove_bot_location_at_index(0);
    println!("Board: {}", bot_map);
    bot_map.print_board();
    bot_map.remove_bot_location_at_index(12);
    println!("Board: {}", bot_map);
    bot_map.print_board();
    bot_map.add_new_bot_at_index(kilobot::new_kilobot(1), 6, 0);
    println!("Board: {}", bot_map);
    bot_map.print_board();
}

fn test_board_controller(board_controller: &mut board_controller::BoardController)
{
    test_bot_map(&mut board_controller.board.bot_map);
    board_controller.move_bot_by_index(6,0);
    println!("Board: {}", board_controller.board.bot_map);
    board_controller.board.bot_map.print_board();
    board_controller.board.bot_map.add_new_bot_at_index(kilobot::new_kilobot(3), 12, 180);
    board_controller.move_bot_by_index(0,4);
    board_controller.move_bot_by_index(12,13);
    board_controller.move_bot_by_index(13,18);
    board_controller.move_bot_by_index(4,9);
    println!("Board: {}", board_controller.board);
    board_controller.board.bot_map.print_board();
    board_controller.board.bot_map.add_new_bot_at_index(kilobot::new_kilobot(4), 6, 135);
    board_controller.move_bot_forward(6);
    board_controller.move_bot_forward(18);
    println!("Board: {}", board_controller.board);
    board_controller.board.bot_map.print_board();
    board_controller.move_bot_forward(23);
    println!("Board: {}", board_controller.board);
    board_controller.board.bot_map.print_board();

}

fn test_signal_map(sig_map: &mut signal_map::SignalMap)
{
    println!("{}", sig_map);
    let mut new_src = SignalSource::new(CoordinatePair::new(0,1),1.5);
    sig_map.add_new_source(new_src);
    sig_map.remove_source_at_coord(&CoordinatePair::new(0,1));
    new_src = SignalSource::new(CoordinatePair::new(2,2),2.0);
    sig_map.add_new_source(new_src);
    sig_map.print_signal_map_to_console();
    let c = sig_map.get_source_at_coord(&CoordinatePair::new(2,2)).ok().unwrap().coord.clone();
    sig_map.move_source_to_coord(&c, &CoordinatePair::new(3,3));
    println!("{}", sig_map);
    sig_map.print_signal_map_to_console();

}



