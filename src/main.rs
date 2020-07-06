use crate::board_controller::BoardController;

mod board_controller;
mod kilobot;
mod board;

fn main() {
    let bot_radius = 5;
    let mut new_bot = kilobot::new_kilobot(0,bot_radius);
    test_bot(&mut new_bot);
    let mut new_board = board::new_board(5, 5);
    //test_board(&mut new_board);
    let mut new_board_controller = &mut BoardController{ board: new_board};
    test_board_controller(new_board_controller);
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
    board.add_new_bot_at_coord(kilobot::new_kilobot(1, 1), 0, 0, 0);
    board.add_new_bot_at_coord(kilobot::new_kilobot(2, 1), 0, 0, 0);
    board.add_new_bot_at_coord(kilobot::new_kilobot(3, 1), 2, 2, 0);
    board.remove_bot_location_at_coord(0,0);
    println!("Board: {}", board);
    board.print_board();
    board.remove_bot_location_at_coord(2,2);
    println!("Board: {}", board);
    board.print_board();
    board.add_new_bot_at_coord(kilobot::new_kilobot(1, 1), 1, 1, 0);
    println!("Board: {}", board);
    board.print_board();
}

fn test_board_controller(board_controller: &mut board_controller::BoardController)
{
    board_controller.board.add_new_bot_at_coord(kilobot::new_kilobot(1, 1), 0, 0, 0);
    board_controller.board.add_new_bot_at_coord(kilobot::new_kilobot(2, 1), 0, 0, 0);
    board_controller.board.add_new_bot_at_coord(kilobot::new_kilobot(3, 1), 2, 2, 0);
    board_controller.board.remove_bot_location_at_coord(0,0);
    println!("Board: {}", board_controller.board);
    board_controller.board.print_board();
    board_controller.board.remove_bot_location_at_coord(2,2);
    println!("Board: {}", board_controller.board);
    board_controller.board.print_board();
    board_controller.board.add_new_bot_at_coord(kilobot::new_kilobot(1, 1), 1, 1, 0);
    println!("Board: {}", board_controller.board);
    board_controller.board.print_board();
    board_controller.move_bot_by_coord(1,1,0,0);
    println!("Board: {}", board_controller.board);
    board_controller.board.print_board();

}



