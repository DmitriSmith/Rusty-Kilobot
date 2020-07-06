mod kilobot;
mod board;

fn main() {
    let bot_radius = 5;
    let mut new_bot = kilobot::new_kilobot(0,bot_radius);
    test_bot(&mut new_bot);
    let mut new_board = board::new_board(5, 5);
    test_board(&mut new_board);
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
    board.add_bot_location(kilobot::new_kilobot(1,1),0,0, 0);
    board.add_bot_location(kilobot::new_kilobot(2,1),0,0, 0);
    board.add_bot_location(kilobot::new_kilobot(3,1),2,2, 0);
    board.remove_bot_location_at_coord(0,0);
    println!("Board: {}", board);
    board.print_board();
    board.remove_bot_location_at_coord(2,2);
    println!("Board: {}", board);
    board.print_board();
    board.add_bot_location(kilobot::new_kilobot(1,1),1,1, 0);
    println!("Board: {}", board);
    board.print_board();
}



