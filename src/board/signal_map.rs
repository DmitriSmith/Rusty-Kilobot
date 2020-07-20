use crate::kilobot::Kilobot;

/// Map of all broadcasts and their range on the board
pub struct SignalMap
{
    width: u8,
    height: u8,
    sources: Vec<Option<Signal>>, //2D array packed into a vector, meant to overlay a BotMap
}

/// Represents a signal source
pub struct Signal
{
    radius: u8,
    source: Kilobot,
}

impl SignalMap
{
    /// Create a new SignalMap and initialize it with None in all locations
    /// # Arguments
    /// * 'width' - How wide the board is
    /// * 'height' - How tall the board is
    ///
    /// For example, new(4,3) would create a 4x3 bot_map that looks like this
    ///             * * * *
    ///             * * * *
    ///             * * * *
    ///         where '*' represents "None"
    pub fn new(width: u8, height: u8) -> SignalMap
    {
        let mut new_map = SignalMap{width, height, sources: Vec::with_capacity((width * height).into())};
        for _i in 0..width * height
        {
            new_map.locations.push(None);
        }
        new_map
    }
}