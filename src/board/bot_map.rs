use crate::kilobot::Kilobot;
use crate::board::{CoordinatePair, LocationError};
use std::{mem, fmt};
use crate::board::board_map::BoardMap;

/// Struct representing the field that Kilobots move on
/// # Fields
/// * 'width' - Width of the board
/// * 'height' - Height of the board
/// * 'locations' - Packed vector of Option<BotLocation> representing each space on the board, where
///  any index that is not null has a bot, and any index that is null has no bot
pub struct BotMap
{
    width: usize,
    height: usize,
    // TODO: Make private, add getter/setter
    pub bots: Vec<Option<BotLocation>>,         //2D array packed into a Vector
}

impl BotMap
{
    /// Create a new BotMap and initialize it with None in all locations
    /// # Arguments
    /// * 'width' - How wide the board is
    /// * 'height' - How tall the board is
    ///
    /// For example, new(4,3) would create a 4x3 bot_map that looks like this
    ///             * * * *
    ///             * * * *
    ///             * * * *
    ///         where '*' represents "None"
    pub fn new(width: usize, height: usize) -> BotMap
    {
        let mut new_map = BotMap{width, height, bots: Vec::with_capacity((width * height).into())};
        for _i in 0..width * height
        {
            new_map.bots.push(None);
        }
        new_map
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
        if index >= self.len()
        {
            Some(LocationError::OutOfBounds);
        }
        match self.bots[index] {
            Some(_) => Some(LocationError::AlreadyOccupied),
            None => {
                mem::swap(&mut self.bots[index], &mut Some(BotLocation { bot, facing }));
                None
            }
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
        if index < self.bots.len()
        {
            match self.bots.get(index).unwrap().as_ref() {
                Some(_) => Some(LocationError::AlreadyOccupied),
                None => {
                    mem::swap(&mut self.bots[index], &mut Some(bot_loc));
                    None
                }
            }
        } else {
            Some(LocationError::OutOfBounds)
        }
    }

    /// Removes the BotLocation at the specified index if a bot is present there and replaces it with None
    /// # Arguments
    /// * 'index' - Index of BotLocation to remove
    /// # Returns
    /// * Ok - Box<BotLocation> Pointer to removed BotLocation
    /// * Err(LocationError) if index is out of bounds or there is no bot in the coordinate
    pub fn remove_bot_location_at_index(&mut self, index: usize) -> Result<Box<BotLocation>,LocationError>
    {
        if index >= 0 && index < self.bots.len()
        {
            match self.bots.get(index)
            {
                Some(b) => {
                    let bot = mem::replace(&mut self.bots[index], None);
                    Ok(Box::new(bot.unwrap()))
                },
                None => Err(LocationError::NotOccupied),
            }

        }
        else { Err(LocationError::OutOfBounds) }
    }

    /// Returns an immutable reference to the bot at given coordinates, or LocationError if none
    /// # Arguments
    /// * 'index' - Index of location in board array
    /// # Returns
    /// * Ok - Reference to a Kilobot
    /// * Err - LocationError if no bot is found, or out of bounds
    pub fn get_bot_at_index(&self, index: usize) -> Result<&Kilobot, LocationError>
    {
        Ok(BotMap::get_bot_location_at_index(self, index)?.bot())
    }

    /// Gets the BotLocation at the given index
    /// # Arguments
    /// * 'index' - Index to get BotLocation from
    /// # Returns
    /// * Ok - Reference to BotLocation at given index
    /// LocationError if None or Out of Bounds
    pub fn get_bot_location_at_index(&self, index: usize) -> Result<&BotLocation, LocationError>
    {
        if index < self.bots.len()
        {

            match self.bots.get(index).unwrap()
            {
                Some(loc) => Ok(loc),
                None => Err(LocationError::NotOccupied),
            }
        }
        else { Err(LocationError::OutOfBounds) }
    }

    /// Returns whether the given index is occupied by a kilobot
    /// # Arguments
    /// * 'index' - Vector index to check
    /// # Returns
    /// * true if the location has a kilobot inside it
    /// * LocationError if the index is out of bounds
    pub fn index_is_occupied(&self, index: usize) -> Result<bool, LocationError>
    {
        if index < self.bots.len()
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
                let this_space = self.bots.get(match self.get_index_from_coord(CoordinatePair{x,y}) {
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

impl BoardMap for BotMap
{
    /// Get the width of the board
/// # Returns
/// * Width of the board
    fn get_width(&self) -> usize
    {
        self.width
    }

    /// Gets the height of the board
    /// # Returns
    /// * Height of the board
    fn get_height(&self) -> usize
    {
        self.height
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

impl fmt::Display for BotMap
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        let mut num_bots: u16 = 0;
        for index in 0..self.bots.len()
        {
            if self.bots.get(index).unwrap().is_some()
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