use crate::board::{Board, LocationError};

pub struct BoardController
{
    pub(crate) board: Board,
}

impl BoardController
{
    /// Moves a BotLocation to a new index on the board
    /// Specifically, gets indices of src and dest and calls move_bot_by_index
    /// # Arguments
    /// * 'src_X' - X coordinate of BotLocation to be moved
    /// * 'src_Y' - X coordinate of BotLocation to be moved
    /// * 'dest_X' - Y coordinate to move BotLocation to
    /// * 'dest_Y' - Y coordinate to move BotLocation to
    /// # Returns
    /// * Option<LocationError> if either coordinate is out of bounds, or if there is no BotLocation
    /// in the source, or if the destination already has a bot
    pub fn move_bot_by_coord(&mut self, src_x: u8, src_y: u8, dest_x: u8, dest_y: u8) -> Option<LocationError>
    {
        match self.board.get_index_from_coord(src_x, src_y)
        {
            Ok(src) => match self.board.get_index_from_coord(dest_x, dest_y)
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
}