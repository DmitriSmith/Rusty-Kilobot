pub const LED_OFF: (u8, u8, u8) = (0, 0, 0);

//Struct representing the kilobot LED
pub struct Led_t
{
    r: u8,
    g: u8,
    b: u8,
}

impl Led_t
{
    //Set the value of an LED
    pub fn set(&mut self, red: u8, green: u8, blue: u8)
    {
        self.r = red;
        self.g = green;
        self.b = blue;
    }


}

//Create a new LED
pub fn new_led(r: u8, g: u8, b: u8) -> Led_t
{
    Led_t {r, g, b}
}