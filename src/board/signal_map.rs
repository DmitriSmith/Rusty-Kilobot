use crate::board::{CoordinatePair, LocationError};
use crate::board::board_map::BoardMap;

/// Map of all broadcasts and their range on the board
pub struct SignalMap
{
    width: usize,
    height: usize,
    sources: Vec<Option<SignalSource>>,   //A map of all signal sources on the board. Overlays a BotMap
    signals: Vec<Signal>, //All spaces that have a readable signal, meant to overlay a BotMap
}

/// Represents a space on the board and stores all readable signals at that space
pub struct Signal
{
    pub sources: Vec<SignalSource>,
}

/// Represents a signal source
pub struct SignalSource
{
    pub radius: u8,
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
    pub fn new(width: usize, height: usize) -> SignalMap
    {
        let len = width * height;
        let mut new_map = SignalMap{width, height, sources: Vec::with_capacity(len), signals: Vec::with_capacity(len)};
        for _i in 0..len
        {
            new_map.sources.push(None);
        }
        new_map
    }
}

impl BoardMap for SignalMap
{
    fn get_width(&self) -> usize {
        self.width
    }

    fn get_height(&self) -> usize {
        self.height
    }
}