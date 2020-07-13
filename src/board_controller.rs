use crate::board::{Board, LocationError, BotLocation, CoordinatePair};

/// Object responsible for manipulating the board
/// # Fields
/// board - Board struct
pub struct BoardController
{
    pub(crate) board: Board,
}

impl BoardController
{
    /// Moves a BotLocation to a new index on the board
    /// Specifically, gets indices of src and dest and calls move_bot_by_index
    /// # Arguments
    /// * 'src_coord' - Coordinates of source BotLocation
    /// * 'dest_coords' - Coordinates to move BotLocation to
    /// # Returns
    /// * Option<LocationError> if either coordinate is out of bounds, or if there is no BotLocation
    /// in the source, or if the destination already has a bot
    pub fn move_bot_by_coord(&mut self, src_coords: CoordinatePair, dest_coords: CoordinatePair) -> Option<LocationError>
    {
        match self.board.get_index_from_coord(src_coords)
        {
            Ok(src) => match self.board.get_index_from_coord(dest_coords)
            {
                Ok(dest) => self.move_bot_by_index(src, dest),
                Err(e) => Some(e)
            },
            Err(e) => Some(e)
        }
    }

    /// Moves a BotLocation to a new index on the board
    /// # Arguments
    /// * 'src_index' - Index of BotLocation to be moved
    /// * 'dest_index' - Index of board to move BotLocation to
    /// # Returns
    /// * Option<LocationError> if either coordinate is out of bounds, or if there is no BotLocation
    /// in the source, or if the destination already has a bot
    pub fn move_bot_by_index(&mut self, src_index: usize, dest_index: usize) -> Option<LocationError>
    {
        if src_index < self.board.len() && dest_index < self.board.len()
        {
            match self.board.index_is_occupied(dest_index)
            {
                Ok(is_occupied) => {
                    match self.board.remove_bot_location_at_index(src_index)
                    {
                        Ok(b) => {
                            if !is_occupied
                            {
                                let bot = *b;
                                self.board.add_bot_location_at_index(bot, dest_index);
                                None
                            } else {
                                Some(LocationError::AlreadyOccupied)
                            }
                        }
                        Err(e) => Some(e),
                    }
                },
                Err(e) => Some(e)
            }
        } else {
            Some(LocationError::OutOfBounds)
        }
    }

    /// Moves the bot forward relative to its current facing
    /// # Arguments
    /// 'index' - Array index of the bot to be moved
    /// # Returns
    /// * Option<LocationError> if the index is out of bounds or doesn't have a bot, or if the
    /// destination is out of bounds or already has a bot
    pub fn move_bot_forward(&mut self, index: usize) -> Option<LocationError>
    {
        match self.board.get_coord_from_index(index)
        {
            Ok(src) => {
                match self.board.get_bot_location_at_index(index)
                {
                    Ok(bot) => {
                        let delta = BoardController::get_forward_coord_delta(bot.get_facing() as f64);

                        //Check bounds of board
                        let dest_x = (src.x as i8) + delta.0;
                        let dest_y = (src.y as i8) + delta.1;
                        if dest_x >= 0 && dest_x < self.board.get_width() as i8
                        {
                            if dest_y >= 0 && dest_y < self.board.get_height() as i8
                            {
                                let dest = CoordinatePair { x: dest_x as u8, y: dest_y as u8 };
                                //This shouldn't call for a match, but the compiler freaked out if I just returned self.move_bot_by_coord
                                return match self.move_bot_by_coord(src, dest)
                                {
                                    Some(e) => Some(e),
                                    None => None
                                }

                            }
                        }
                        Some(LocationError::OutOfBounds)

                    },
                    Err(e) => Some(e)
                }
            },
            Err(e) => Some(e)
        }

    }

    /// Gets the change in coordinates for a bot moving forward
    /// # Arguments
    /// * 'facing' - the direction the bot is facing in degrees clockwise from north
    /// # Returns
    /// * (i8, i8) - The change in coordinates if the bot were to move forward.
    /// Note that this is the change relative to the bot's current location,
    /// *not* the final coordinates of the move
    pub fn get_forward_coord_delta(facing: f64) -> (i8, i8)
    {
        let delta_x = facing.to_radians().sin().round() as i8;
        let delta_y = -1 * facing.to_radians().cos().round() as i8;
        (delta_x,delta_y)
    }
}