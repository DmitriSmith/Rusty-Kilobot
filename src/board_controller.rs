use crate::board::{LocationError, CoordinatePair, Board};
use crate::board::board_map::BoardMap;

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
            match self.board.bot_map.index_is_occupied(dest_index)
            {
                Ok(is_occupied) => {
                    match self.board.bot_map.remove_bot_location_at_index(src_index)
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
    pub fn move_bot_forward(&mut self, src: usize) -> Option<LocationError>
    {
        match self.board.get_coord_from_index(src)
        {
            Ok(src_coord) => {
                match self.board.get_bot_location_at_index(src)
                {
                    Ok(bot) => {
                        let delta = BoardController::get_forward_coord_delta(bot.get_facing() as f64);

                        //Check bounds of board
                        let dest_x = (src_coord.x as i8) + delta.0;
                        let dest_y = (src_coord.y as i8) + delta.1;
                        if dest_x >= 0 && dest_x < self.board.get_width() as i8
                        {
                            if dest_y >= 0 && dest_y < self.board.get_height() as i8
                            {
                                let dest = self.board.get_index_from_coord(CoordinatePair { x: dest_x as usize, y: dest_y as usize }).ok().unwrap();
                                //This shouldn't call for a match, but the compiler freaked out if I just returned self.move_bot_by_coord
                                return match self.move_bot_by_index(src, dest)
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
            }
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