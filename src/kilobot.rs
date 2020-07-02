use std::fmt;

mod led_t;

pub const MOTOR_MAX_VAL: u8 = 255;

//Struct representing the kilobot
/*
 * Kilobot is goverened by physical limitations
 * Motors are pwm, left & right motor values represent duty cycle of signal to motors
 * TODO: Move to separate module
 */
pub struct Kilobot
{
    left_motor: u8,
    right_motor: u8,
    led: led_t::Led_t,
    uid: u32,
    message_received: bool,
    radius: u16,
}

impl Kilobot
{
    /*
    Kilobots can do the following:
        Turn right
        Turn left
        Move forward
        Use transceiver - Should always be receiving in background
        Use LED

        Turns by spinning one motor
    */

    //Turn the kilobot left
    pub fn turn_left(&mut self)
    {
        self.set_motors(0,MOTOR_MAX_VAL);
    }

    //Turn the kilobot right
    pub fn turn_right(&mut self)
    {
        self.set_motors(MOTOR_MAX_VAL,0);
    }

    //Move straight forward
    pub fn move_forward(&mut self)
    {
        self.set_motors(MOTOR_MAX_VAL,MOTOR_MAX_VAL);
    }

    //Stop moving
    pub fn stop(&mut self)
    {
        self.set_motors(0,0);
    }

    //Sets the motors
    pub fn set_motors(&mut self, left_m: u8, right_m: u8)
    {
        self.left_motor = left_m;
        self.right_motor = right_m;
    }

    //Set the color of the LED
    pub fn set_led(&mut self, r: u8, g: u8, b: u8)
    {
        self.led.set(r,g,b);
    }

    //Returns the raw motor values formatted as (left_motor, right_motor)
    pub fn get_motor_values(&self) -> (u8, u8)
    {
        (self.left_motor, self.right_motor)
    }

}

impl fmt::Display for Kilobot
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(UID:{}, Message Received:{}, left motor:{}, right motor:{})"
               , self.uid
               , self.message_received.to_string()
               , self.left_motor
               , self.right_motor)
    }
}

//Create a new kilobot
pub fn new_kilobot(uid: u32, radius: u16) -> Kilobot
{
    Kilobot {left_motor: 0, right_motor: 0, led: led_t::new_led(0,0,0), uid, message_received: false, radius}
}
