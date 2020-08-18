/*
 *
 *  Bresenham's algorithm code taken from here
 *  https://www.redblobgames.com/grids/circle-drawing/
 *
 */

use crate::board::{CoordinatePair, LocationError};
use crate::board::board_map::BoardMap;
use std::{mem, fmt};
use std::cmp::{max, min};

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
    pub coord: CoordinatePair,
    pub radius: f64,
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
            new_map.signals.push(Signal{ sources: vec![] });
        }
        new_map
    }

    /// Add a new signal source to the signal_map and fill the area that the signal reaches
    /// # Arguments
    /// * 'source' - New source to add to the board
    /// * 'index' - Index of coordinate new source is located at
    /// # Returns
    /// LocationError if index is out of bounds or already occupied, or None if successful
    pub fn add_new_source(&mut self, source: SignalSource) -> Option<LocationError>
    {
        match self.get_index_from_coord(&source.coord)
        {
            Ok(index) => {
                match self.sources[index]
                {
                    Some(_) => Some(LocationError::AlreadyOccupied),
                    None => {
                        self.fill_circle(&source, Signal::add_source);
                        mem::swap(&mut self.sources[index], &mut Some(source));
                        None
                    },
                }
            },
            Err(e) => Some(e),
        }
    }

    /// Remove a signal source from the signal map
    /// # Arguments
    /// * 'coord' - Coordinates of SignalSource to be removed
    /// # Returns
    /// The signal source that was removed if successful, LocationError if out of bounds or no source at coordinates
    pub fn remove_source_at_coord(&mut self, coord: &CoordinatePair) -> Result<SignalSource,LocationError>
    {
        match self.get_index_from_coord(coord)
        {
            Ok(index) => {
                match self.sources.get(index)
                {
                    Some(_s) => {
                        let mut src = mem::replace(&mut self.sources[index], None).unwrap();
                        self.fill_circle(&src, Signal::remove_source);
                        Ok(src)
                    },
                    None => Err(LocationError::NotOccupied),
                }
            },
            Err(e) => Err(e),
        }
    }

    /// Get a reference to the SignalSource at the provided coordinates
    /// # Arguments
    /// * 'coord' - Coordinate on SignalMap to get SignalSource from
    /// # Returns
    /// Reference to a SignalSource at the given coordinate, or LocationError if out of bounds or no source present
    pub fn get_source_at_coord(&self, coord: &CoordinatePair) -> Result<&SignalSource,LocationError>
    {
        match self.get_index_from_coord(coord)
        {
            Ok(index) => {
                match &self.sources[index]
                {
                    Some(src) => Ok(src),
                    None => Err(LocationError::NotOccupied),
                }
            },
            Err(e) => Err(e),
        }
    }

    /// Get a mutable reference to the SignalSource at the provided coordinates
    /// # Arguments
    /// * 'coord' - Coordinate on SignalMap to get SignalSource from
    /// # Returns
    /// Mutable reference to a SignalSource at the given coordinate, or LocationError if out of bounds or no source present
    pub fn get_mut_source_at_coord(&mut self, coord: &CoordinatePair) -> Result<&mut SignalSource,LocationError>
    {
        match self.get_index_from_coord(coord)
        {
            Ok(index) => {
                match self.sources.get_mut(index).unwrap()
                {
                    Some(src) => Ok(src),
                    None => Err(LocationError::NotOccupied),
                }
            },
            Err(e) => Err(e),
        }
    }

    /// Move a source to a destination CoordinatePair.
    /// !! Note that src must already be in a valid location on the SignalMap !!
    /// # Arguments
    /// * 'src' - SignalSource to move. Must already be in a valid location on the SignalMap
    /// * 'dest' - Destination to try to move src to
    /// # Returns
    /// * None if the move was successful, LocationError if any snags were hit
    pub fn move_source_to_coord(&mut self, src: &CoordinatePair, dest: &CoordinatePair) -> Option<LocationError>
    {
        match self.get_index_from_coord(&src)
        {
            Ok(_) => {
                match self.get_index_from_coord(&dest)
                {
                    Ok(dest_index) => {
                        match self.sources.get_mut(dest_index).unwrap()
                        {
                            Some(_x) => Some(LocationError::AlreadyOccupied),
                            None => {
                                match self.remove_source_at_coord(src)
                                {
                                    Ok(mut removed_src) => {
                                        removed_src.coord = dest.clone();
                                        self.add_new_source(removed_src);
                                        None
                                    },
                                    Err(e) => Some(e),
                                }
                            }
                        }
                    },
                    Err(e) => Some(e),
                }
            },
            Err(e) => Some(e),
        }

    }

    /// Helper function to determine whether a square is within a circle
    /// https://www.redblobgames.com/grids/circle-drawing/
    /// # Arguments
    /// * 'center' - index of the signal source
    /// * 'target' - Square to check
    /// * 'radius' - radius of the circle
    /// # Returns
    /// True if target is within the circle, or false if outside or out of bounds
    fn inside_circle(center: &CoordinatePair, target: &CoordinatePair, radius: f64) -> bool
    {
        let c = center.as_f64_tuple();
        let t = target.as_f64_tuple();
        let dx = c.0 - t.0;
        let dy = c.1 - t.1;
        let distance_squared = dx*dx + dy*dy;
        distance_squared <= radius*radius
    }

    /// Helper function to get the bounding box of a circle. !!Does not check if center is out of bounds!!
    /// # Arguments
    /// * 'center' - The coordinates of the center of the circle
    /// * 'radius' - the radius of the circle
    /// # Returns
    /// (top, bottom, left, right) - The boundaries of the circle as tuple
    fn get_bounding_box(&self, center: &CoordinatePair, radius: f64) -> (usize,usize,usize,usize)
    {
        let top: usize = min(0, (center.y as f64 - radius) as usize);
        let bottom: usize = max(self.height, (center.y as f64 + radius) as usize);
        let left: usize = min(0, (center.x as f64 - radius) as usize);
        let right: usize = max(self.width, (center.x as f64 + radius) as usize);

        (top, bottom, left, right)
    }

    /// Perform a function on each space inside a circle originating from SignalSource
    /// # Arguments
    /// * 'src' - SignalSource that is origin of circle
    /// * 'func' - Function to use to modify each space inside the circle
    fn fill_circle<F>(&mut self, src: &SignalSource, func: F) where F: Fn(&mut Signal, &SignalSource)
    {
        let center = CoordinatePair::new(src.coord.x,src.coord.y);
        let bounding_box = self.get_bounding_box(&center, src.radius);
        for y in bounding_box.0..bounding_box.1
        {
            for x in bounding_box.2..bounding_box.3
            {
                let target = CoordinatePair {x, y};
                if SignalMap::inside_circle(&center, &target, src.radius)
                {
                    match self.get_index_from_coord(&target)
                    {
                        Ok(i) => func(self.signals.get_mut(i).unwrap(), src),
                        Err(_e) => continue,
                    }
                }
            }
        }
    }

    pub fn print_signal_map_to_console(&self)
    {
        for index in 0..self.len()
        {
            let mut txt: String;
            if self.sources[index].is_some()
            {
                txt = "O".parse().unwrap();
            } else if self.signals[index].sources.len() > 0
            {
                txt = "#".parse().unwrap();
            } else {
                txt = "~".parse().unwrap();
            }
            if index % self.width == self.width - 1
            {
                txt = format!("{}{}", txt, "\n");
                print!("{}", txt);
            } else {
                print!("{} ", txt);
            }
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
    /// * 'index' - Array index of the signal source
    /// * 'radius' - Broadcast radius of the signal in board units
    /// # Returns
    /// New, initialized SignalSource object
    pub fn new(coord: CoordinatePair, radius: f64) -> SignalSource
    {
        SignalSource{ coord, radius }
    }

    pub fn get(&self) -> &SignalSource
    {
        self
    }

    pub fn get_mut(&mut self) -> &mut SignalSource
    {
        self
    }
}

impl Signal
{
    /// Add a detectable signal source, sort, then remove any duplicates
    /// Sort is needed to ensure that deduping catches all duplicates
    /// # Arguments
    /// * 'src' - Coordinates of the source
    pub fn add_source(point: &mut Signal, src: &SignalSource)
    {
        point.sources.push(src.coord.as_u8_tuple());
        point.sources.sort_by_key(|k| (k.0, k.1));
        point.sources.dedup();
    }

    /// Removes the given signal source origin if present
    /// # Arguments
    /// * 'src' - Origin coordinates to remove
    pub fn remove_source(point: &mut Signal, src: &SignalSource)
    {
        point.sources.retain(|&x| x != src.coord.as_u8_tuple())
    }

}

impl fmt::Display for SignalMap
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        let mut num_srcs: u16 = 0;
        for index in 0..self.len()
        {
            if self.sources[index].is_some()
            {
                num_srcs += 1;
            }
        }
        write!(f, "(width:{}, height:{}, number of srcs:{})"
               , self.width
               , self.height
               , num_srcs)
    }
}