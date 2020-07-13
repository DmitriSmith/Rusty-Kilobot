use crate::kilobot ::*;
use std::{fmt, mem};
use crate::board::LocationError::OutOfBounds;

pub const NORTH: u16 = 0;
pub const EAST: u16 = 90;
pub const SOUTH: u16 = 180;
pub const WEST: u16 = 270;

/// Basic error tpe that encompasses errors that can occur related to the board.
/// Doesn't carry any sort of message
pub enum LocationError {
    AlreadyOccupied,
    NotOccupied,
    OutOfBounds,
}

/// Struct representing an (x,y) coordinate on a 2D plane
pub struct CoordinatePair
{
    pub(crate) x: u8,
    pub(crate) y: u8,
}

/// Struct representing the field that Kilobots move on
/// # Fields
/// * 'width' - Width of the board
/// * 'height' - Height of the board
/// * 'locations' - Packed vector of Option<BotLocation> representing each space on the board, where
///  any index that is not null has a bot, and any index that is null has no bot
pub struct Board
{
    width: u8,
    height: u8,
    locations: Vec<Option<BotLocation>>,         //2D array packed into a Vector
}

impl Board
{
    /// Add new bot to the board at the given coordinates
    /// # Arguments
    /// 'bot' - Kilobot to add to the board
    /// 'coord' - CoordinatePair to place bot at
    /// 'facing' - Direction the bot is initially facing, in degrees clockwise from north
    /// # Returns
    /// None - Insert successful
    /// LocationError if out of bounds or coordinates already occupied
    pub fn add_new_bot_at_coord(&mut self, bot: Kilobot, coord: CoordinatePair, facing: u16) -> Option<LocationError>
    {
        match self.get_index_from_coord(coord)
        {
            Ok(index) => self.add_new_bot_at_index(bot, index, facing),
            Err(e) => Some(e)
        }
    }

    /// Add new bot to the board at the given index
    /// # Arguments
    /// 'bot' - Kilobot to add to the board
    /// 'index' - Index in vector to place the bot
    /// 'facing' - Direction the bot is initially facing, in degrees clockwise from north
    /// # Returns
    /// None - Insert successful
    /// LocationError if out of bounds or coordinates already occupied
    pub fn add_new_bot_at_index(&mut self, bot: Kilobot, index: usize, facing: u16) -> Option<LocationError>
    {
        match self.locations.get(index).unwrap().as_ref() {
            Some(_) => Some(LocationError::AlreadyOccupied),
            None => {
                mem::swap(&mut self.locations[index], &mut Some(BotLocation { bot, facing }));
                None
            }
        }
    }

    /// Adds an existing BotLocation to the given coordinate
    /// # Arguments
    /// * 'bot_loc' - Existing BotLocation object
    /// * 'coord' - Coordinate to place bot location into
    /// # Returns
    /// Option<LocationError> if coordinates are out of bounds, or there is already a bot at the coordinates
    pub fn add_bot_location_at_coord(&mut self, bot_loc: BotLocation, coord: CoordinatePair) -> Option<LocationError>
    {
        match self.get_index_from_coord(coord)
        {
            Ok(index) => self.add_bot_location_at_index(bot_loc, index),
            Err(e) => Some(e)
        }
    }

    /// Adds an existing BotLocation to the given index
    /// # Arguments
    /// * 'bot_loc' - Existing BotLocation object
    /// * 'index' - Index to insert into
    /// # Returns
    /// Option<LocationError> if coordinates are out of bounds, or there is already a bot at the coordinates
    pub fn add_bot_location_at_index(&mut self, bot_loc: BotLocation, index: usize) -> Option<LocationError>
    {
        if index < self.locations.len()
        {
            match self.locations.get(index).unwrap().as_ref() {
                Some(_) => Some(LocationError::AlreadyOccupied),
                None => {
                    mem::swap(&mut self.locations[index], &mut Some(bot_loc));
                    None
                }
            }
        } else {
            Some(LocationError::OutOfBounds)
        }
    }

    /// Removes the BotLocation at the specified coordinates if a bot is present there and replaces it with None
    /// Finds the index of the coordinate pair and calls remove_bot_location_at_index
    /// # Arguments
    /// * 'coord' - Coordinates of BotLocation to remove
    /// # Returns
    /// * Ok - Box<BotLocation> Pointer to removed BotLocation
    /// * Err(LocationError) if coordinates are out of bounds or there is no bot in the coordinate
    pub fn remove_bot_location_at_coord(&mut self, coord: CoordinatePair) -> Result<Box<BotLocation>,LocationError>
    {
            self.remove_bot_location_at_index(self.get_index_from_coord(coord)?)
    }

    /// Removes the BotLocation at the specified index if a bot is present there and replaces it with None
    /// # Arguments
    /// * 'index' - Index of BotLocation to remove
    /// # Returns
    /// * Ok - Box<BotLocation> Pointer to removed BotLocation
    /// * Err(LocationError) if index is out of bounds or there is no bot in the coordinate
    pub fn remove_bot_location_at_index(&mut self, index: usize) -> Result<Box<BotLocation>,LocationError>
    {
        if index >= 0 && index < self.locations.len()
        {
            match self.locations.get(index)
            {
                Some(b) => {
                    let bot = mem::replace(&mut self.locations[index], None);
                    Ok(Box::new(bot.unwrap()))
                },
                None => Err(LocationError::NotOccupied),
            }

        }
        else { Err(LocationError::OutOfBounds) }
    }

    /// Returns an immutable reference to the bot at given coordinates, or LocationError if none
    /// Finds the index of the coordinates then calls get_bot_at_index(index)
    /// # Arguments
    /// * 'coord' - Coordinate to get bot from
    /// # Returns
    /// * Ok - Reference to Box<Kilobot>
    /// * Err - LocationError if no bot is found, or out of bounds
    pub fn get_bot_at_coord(&self, coord: CoordinatePair) -> Result<&Kilobot, LocationError>
    {
        self.get_bot_at_index(self.get_index_from_coord(coord)?)
    }

    /// Returns an immutable reference to the bot at given coordinates, or LocationError if none
    /// # Arguments
    /// * 'index' - Index of location in board array
    /// # Returns
    /// * Ok - Reference to a Kilobot
    /// * Err - LocationError if no bot is found, or out of bounds
    pub fn get_bot_at_index(&self, index: usize) -> Result<&Kilobot, LocationError>
    {
        Ok(Board::get_bot_location_at_index(self, index)?.bot())
    }

    /// Gets the BotLocation at the given coordinates
    /// # Arguments
    /// * 'coord' - Coordinates to get BotLocation from
    /// # Returns
    /// * Ok - Reference to BotLocation at given coordinates
    /// LocationError if None or Out of Bounds
    pub fn get_bot_location_at_coord(&self, coord: CoordinatePair) -> Result<&BotLocation, LocationError>
    {
        self.get_bot_location_at_index(self.get_index_from_coord(coord)?)
    }

    /// Gets the BotLocation at the given index
    /// # Arguments
    /// * 'index' - Index to get BotLocation from
    /// # Returns
    /// * Ok - Reference to BotLocation at given index
    /// LocationError if None or Out of Bounds
    pub fn get_bot_location_at_index(&self, index: usize) -> Result<&BotLocation, LocationError>
    {
        if index < self.locations.len()
        {

            match self.locations.get(index).unwrap()
            {
                Some(loc) => Ok(loc),
                None => Err(LocationError::NotOccupied),
            }
        }
        else { Err(LocationError::OutOfBounds) }
    }


    /// Get the array index from an x and y coordinate
    /// # Arguments
    /// * 'x' - X coordinate
    /// * 'y' - Y coordinate
    /// # Returns
    /// * Ok - usize index of desired x & y coordinate
    /// * Err - LocationError if coordinates are out of bounds
    pub fn get_index_from_coord(&self, coord: CoordinatePair) -> Result<usize, LocationError>
    {
        if coord.x < self.width && coord.y < self.height
        {
            Ok((coord.x + (coord.y * self.width)) as usize)
        }
        else { Err(LocationError::OutOfBounds) }

    }

    /// Gets the x & y coordinates from an array index
    /// # Arguments
    /// * 'index' - The array index to convert
    /// # Returns
    /// * Ok - Coordinate pair corresponding with index
    /// * LocationError if index is out of bounds
    pub fn get_coord_from_index(&self, index: usize) -> Result<CoordinatePair, LocationError>
    {
        if index < self.locations.len()
        {
            let x = (index as u8) % self.width;
            let y = ((index as u8) - x) / self.width;
            Ok(CoordinatePair{x,y})
        } else {
            Err(LocationError::OutOfBounds)
        }
    }

    /// Get the length of the vector representing the board
    /// # Returns
    /// * u16 length of the board
    pub fn len(&self) -> usize
    {
        self.locations.len()
    }

    /// Get the width of the board
    /// # Returns
    /// * Width of the board
    pub fn get_width(&self) -> u8
    {
        self.width
    }

    /// Gets the height of the board
    /// # Returns
    /// * Height of the board
    pub fn get_height(&self) -> u8
    {
        self.height
    }

    /// Returns whether the given coordinate pair is occupied by a kilobot
    /// Actually just finds the index of the coordinate pair then call index_is_occupied
    /// # Arguments
    /// * 'coord' - Coordinate to check
    /// # Returns
    /// * true if the location has a kilobot inside it
    /// * LocationError if the coordinates are out of bounds
    pub fn coord_is_occupied(&self, coord: CoordinatePair) -> Result<bool, LocationError>
    {
        self.index_is_occupied(self.get_index_from_coord(coord)?)
    }

    /// Returns whether the given index is occupied by a kilobot
    /// # Arguments
    /// * 'index' - Vector index to check
    /// # Returns
    /// * true if the location has a kilobot inside it
    /// * LocationError if the index is out of bounds
    pub fn index_is_occupied(&self, index: usize) -> Result<bool, LocationError>
    {
        if index < self.locations.len()
        {
            Ok(self.get_bot_at_index(index).is_ok())
        }
        else { Err(LocationError::OutOfBounds) }

    }

    /// Print left to right, top to bottom
    pub fn print_board(&self)
    {
        for y in 0..self.height
        {
            for x in 0..self.width
            {
                let this_space = self.locations.get(match self.get_index_from_coord(CoordinatePair{x,y}) {
                    Ok(index) => index,
                    Err(_) => unimplemented!(),     //Shouldn't be able to get here!
                }).unwrap();
                match this_space
                {
                    Some(loc) => print!("  {}   ", loc.bot.get_uid()),
                    None => print!("({},{}) ", x, y),
                }
            }
            print!("\n\n");
        }
    }
}

/// Struct representing a space that a Kilobot occupies
/// # Fields
/// * 'bot' - Kilobot at this location
/// * 'facing'
pub struct BotLocation
{
    bot: Kilobot,
    facing: u16,            //Represents the current angle of the bot, where 0 is north
}

impl BotLocation
{
    /// Return an immutable reference to the bot in the location
    /// Allows accessing the bot functions, but cannot change bot values
    /// # Returns
    /// * Immutable reference to the bot in the Location
    pub fn bot(&self) -> &Kilobot
    {
        &self.bot
    }

    /// Return a mutable reference to the bot in the location
    /// Allows accessing the bot functions and changing bot values
    /// # Returns
    /// * Mutable reference to the bot in the Location
    pub fn bot_mut(&mut self) -> &mut Kilobot
    {
        &mut self.bot
    }

    /// Return the facing of the bot in the location
    /// # Returns
    /// * The rotation of the bot in degrees clockwise away from north
    pub fn get_facing(&self) -> u16
    {
        self.facing
    }

    /// Sets the facing of the bot in the location in degrees clockwise from north
    /// # Arguments
    /// * 'new_facing' - The new facing of the bot, in degrees clockwise from north
    pub fn set_facing(&mut self, mut new_facing: i16)
    {
        new_facing = new_facing % 360;
        if new_facing < 0
        {
            self.facing = (new_facing + 360) as u16
        } else {
            self.facing = new_facing as u16
        }
    }
}

/// Create a new instance of Board and fill it with empty Locations
/// # Arguments
/// * 'width' - How wide the board should be
/// * 'height' - How tall the board should be
///
/// For example, new_board(4,3) would create a 4x3 board that looks like this
///             * * * *
///             * * * *
///             * * * *
///         where '*' represents "None"
pub fn new_board(width: u8, height: u8) -> Board
{
    let mut board = Board {width, height, locations: Vec::with_capacity((width * height).into())};
    for _i in 0..width * height
    {
        board.locations.push(None);
    }
    return board;
}

impl fmt::Display for Board
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        let mut num_bots: u16 = 0;
        for index in 0..self.locations.len()
        {
            if self.locations.get(index).unwrap().is_some()
            {
                num_bots += 1;
            }
        }
        write!(f, "(width:{}, height:{}, number of bots:{})"
               , self.width
               , self.height
               , num_bots)
    }
}

impl fmt::Display for BotLocation
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f, "[Bot: {}, Facing: {}]"
                , self.bot
                , self.facing)
    }
}