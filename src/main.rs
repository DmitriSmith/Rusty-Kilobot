use crate::kilobot::Kilobot;

mod kilobot;

fn main() {

    println!("Hello, world!");

    let bot_radius = 5;
    let mut new_bot = kilobot::new_kilobot(0,bot_radius);
    test_bot(&mut new_bot);

}

fn test_bot(bot: &mut kilobot::Kilobot)
{
    println!("Bot: {}", bot);
    assert_eq!(bot.get_motor_values(), (0,0));
    bot.move_forward();
    assert_eq!(bot.get_motor_values(), (kilobot::MOTOR_MAX_VAL, kilobot::MOTOR_MAX_VAL));

}


