use crate::map_matrix::MapMatrix;
use crate::map_matrix::MapObject;
use crate::objects::map_object::Rollable;
use crate::objects::map_object::Updateable;

#[derive(Copy, Clone)]
pub struct Diamond {}

impl Updateable for Diamond {
    fn update(&self, map: &mut MapMatrix, x_pos: u8, y_pos: u8) {
        let under = map.get(x_pos, y_pos + 1);
        match under {
            MapObject::Empty => {
                map.set(x_pos, y_pos + 1, MapObject::Diamond(*self));
                map.set(x_pos, y_pos, MapObject::Empty);
                map.set_updated(x_pos, y_pos);
                map.set_updated(x_pos, y_pos + 1);
            }
            MapObject::Boulder(_) | MapObject::Diamond(_) | MapObject::Wall => {
                self.roll(map, x_pos, y_pos);
            }
            _ => (),
        }
    }
}

impl Rollable for Diamond {
    fn roll(&self, map: &mut MapMatrix, x_pos: u8, y_pos: u8) {
        let right = map.get(x_pos + 1, y_pos);
        let right_under = map.get(x_pos + 1, y_pos + 1);
        if matches!(right, MapObject::Empty) && matches!(right_under, MapObject::Empty) {
            map.set(x_pos + 1, y_pos, MapObject::Diamond(*self));
            map.set(x_pos, y_pos, MapObject::Empty);
            map.set_updated(x_pos + 1, y_pos);
            map.set_updated(x_pos, y_pos);
            return;
        }
        let left = map.get(x_pos - 1, y_pos);
        let left_under = map.get(x_pos - 1, y_pos + 1);
        if matches!(left, MapObject::Empty) && matches!(left_under, MapObject::Empty) {
            map.set(x_pos - 1, y_pos, MapObject::Diamond(*self));
            map.set(x_pos, y_pos, MapObject::Empty);
            map.set_updated(x_pos - 1, y_pos);
            map.set_updated(x_pos, y_pos);
        }
    }
}
