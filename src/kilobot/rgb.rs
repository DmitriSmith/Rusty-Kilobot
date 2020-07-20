pub const LED_OFF: (u8, u8, u8) = (0, 0, 0);

/// Struct representing the kilobot LED
pub struct RGB
{
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RGB
{
    /// Set the value of an LED
    /// # Arguments
    /// * 'color' - RGB values to set LED to
    pub fn set(&mut self, color: RGB)
    {
        self.r = color.r;
        self.g = color.g;
        self.b = color.b;
    }

    /// Returns reference to itself
    pub fn get(&self) -> &RGB
    {
        self
    }

}

//Create a new LED
pub fn new_led(r: u8, g: u8, b: u8) -> RGB
{
    RGB {r, g, b}
}