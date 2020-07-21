/*
 *
 *  Bresenham's algorithm code taken from here
 *  https://www.redblobgames.com/grids/circle-drawing/
 *
 */

use crate::board::{CoordinatePair, LocationError};
use crate::board::board_map::BoardMap;
use std::mem;

/// Map of all broadcasts and their range on the board
pub struct SignalMap
{
    width: usize,
    height: usize,
    sources: Vec<Option<SignalSource>>,   //A map of all signal sources on the board. Overlays a BotMap
    signals: Vec<Signal>, //All spaces that have a readable signal, meant to overlay a BotMap
}

/// Represents a space on the board and stores all readable signals at that space
/// Sources are represented by the coordinate they originate from, since rust doesn't like vectors of references
pub struct Signal
{
    pub sources: Vec<(u8, u8)>,
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

    /// Add a new signal source to the signal_map
    /// # Arguments
    /// * 'source' - New source to add to the board
    /// * 'coord' - Coordinate new source is located at
    /// # Returns
    /// LocationError if index is out of bounds or already occupied, or None if successful
    pub fn add_new_source(&mut self, source: SignalSource, coord: CoordinatePair) -> Option<LocationError>
    {
        let mut index: usize = 0;
        match self.get_index_from_coord(coord)
        {
            Ok(i) => {
                index = i;
                match self.sources[index]
                {
                    Some(_) => Some(LocationError::AlreadyOccupied),
                    None => {
                        mem::swap(&mut self.sources[index], &mut Some(source));

                        None
                    },
                }
            },
            Err(e) => Some(e),
        }
    }

    /// Remove a signal source from the signal map
    pub fn remove_source_at_index(&mut self, index: usize) -> Result<SignalSource,LocationError>
    {
        if index >= self.len()
        {
            return Err(LocationError::OutOfBounds);
        }
        match self.sources.get(index)
        {
            Some(_) => {
                let src = mem::replace(&mut self.sources[index], None);
                Ok(src.unwrap())
            },
            None => Err(LocationError::NotOccupied),
        }
    }

    /// Remove all signals from a source. Builds a bounding box then checks it.
    /// Taken from https://www.redblobgames.com/grids/circle-drawing/
    pub fn remove_signals(&mut self, src_index: usize, src_signal: &SignalSource)
    {

    }

    /// Helper function to determine whether a square is within a circle
    /// https://www.redblobgames.com/grids/circle-drawing/
    /// # Arguments
    /// * 'center' - index of the signal source
    /// * 'target' - Square to check
    /// * 'radius' - radius of the circle
    /// # Returns
    /// True if target is within the circle, or false if outside or out of bounds
    fn inside_circle(&self, center: usize, target: usize, radius: u8) -> bool
    {
        match self.get_coord_from_index(center)
        {
            Ok(c) => {
                match self.get_coord_from_index(target)
                {
                    Ok(t) => {
                        let dx = c.x - t.x;
                        let dy = c.y - t.y;
                        let distance_squared = dx*dx + dy*dy;
                        distance_squared <= radius as usize
                    }
                    Err(e) => false,
                }
            },
            Err(e) => false,
        }
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

impl SignalSource
{
    /// Create a new SignalSource
    /// # Arguments
    /// * 'radius' - Broadcast radius of the signal in board units
    /// # Returns
    /// New, initialized SignalSource object
    pub fn new(&mut self, radius: u8) -> SignalSource
    {
        SignalSource(radius)
    }
}

impl Signal
{
    /// Return a reference to each of the sources broadcasting on this space
    /// # Returns
    /// Vector of all signal sources detectable at this point
    pub fn get_sources(&self) -> &Vec<SignalSource>
    {
        self.sources.as_ref()
    }

    /// Add a detectable signal source, sort, then remove any duplicates
    /// Sort is needed to ensure that deduping catches all duplicates
    /// # Arguments
    /// * 'src' - Coordinates of the source
    pub fn add_source(&mut self, src: (u8, u8))
    {
        self.sources.push(src);
        self.sources.sort_by_key(|k| (k.0, k.1));
        self.sources.dedup();
    }

    /// Removes the given signal source origin if present
    /// # Arguments
    /// * 'src' - Origin coordinates to remove
    pub fn remove_source(&mut self, src: (u8, u8))
    {
        self.sources.retain(|&x| x != src)
    }

}