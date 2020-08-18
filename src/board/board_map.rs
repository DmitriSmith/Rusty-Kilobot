use crate::board::{CoordinatePair, LocationError};

pub trait BoardMap
{
    /// Get the width of the board
    /// # Returns
    /// * Width of the board
    fn get_width(&self) -> usize;

    /// Gets the height of the board
    /// # Returns
    /// * Height of the board
    fn get_height(&self) -> usize;

    /// Get the array index from an x and y coordinate
    /// # Arguments
    /// * 'x' - X coordinate
    /// * 'y' - Y coordinate
    /// # Returns
    /// * Ok - usize index of desired x & y coordinate
    /// * Err - LocationError if coordinates are out of bounds
    fn get_index_from_coord(&self, coord: &CoordinatePair) -> Result<usize, LocationError>
    {
        let width = self.get_width();
        if coord.x < width && coord.y < self.get_height()
        {
            Ok(coord.x + (coord.y * width))
        }
        else { Err(LocationError::OutOfBounds) }

    }

    /// Gets the x & y coordinates from an array index
    /// # Arguments
    /// * 'index' - The array index to convert
    /// # Returns
    /// * Ok - Coordinate pair corresponding with index
    /// * LocationError if index is out of bounds
    fn get_coord_from_index(&self, index: &usize) -> Result<CoordinatePair, LocationError>
    {
        let width = self.get_width();
        if index < &self.len()
        {
            let x = index % width;
            let y = (index - x) / width;
            Ok(CoordinatePair{x,y})
        } else {
            Err(LocationError::OutOfBounds)
        }
    }

    /// Returns the length of the Vector representing the board
    fn len(&self) -> usize
    {
        self.get_width() * self.get_height()
    }
}