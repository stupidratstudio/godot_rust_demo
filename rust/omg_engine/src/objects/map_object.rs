use crate::map_matrix::MapMatrix;

pub trait Updateable {
    fn update(&self, map: &mut MapMatrix, x_pos: u8, y_pos: u8);
}

pub trait Rollable {
    fn roll(&self, map: &mut MapMatrix, x_pos: u8, y_pos: u8);
}
