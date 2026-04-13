use crate::objects::boulder::Boulder;
use crate::objects::diamond::Diamond;
use crate::objects::map_object::Updateable;
use rand::prelude::*;
use std::fmt;

pub struct MapObjectProperties {
    x: u8,
    y: u8,
    r: u8,
    x_o: u8,
    y_o: u8,
}

#[derive(Copy, Clone)]
pub enum MapObject {
    Empty,
    Wall,
    Dirt,
    Player,
    Boulder(Boulder),
    Diamond(Diamond),
}

pub struct MapMatrix {
    map: Box<[MapObject]>,
    updated: Box<[bool]>,
    width: u8,
    height: u8,
}

impl MapMatrix {
    pub fn new(width: u8, height: u8) -> Self {
        let map_size = (width as usize) * (height as usize);
        let map = vec![MapObject::Empty; map_size].into_boxed_slice();
        let updated = vec![false; map_size].into_boxed_slice();
        MapMatrix {
            map,
            updated,
            width,
            height,
        }
    }

    pub fn init(&mut self) {
        for x_pos in 0..self.width {
            self.set(x_pos, 0, MapObject::Wall);
            self.set(x_pos, self.height - 1, MapObject::Wall);
        }
        for y_pos in 0..self.height {
            self.set(0, y_pos, MapObject::Wall);
            self.set(self.width - 1, y_pos, MapObject::Wall);
        }
        let mut rng = rand::rng();
        for y_pos in 1..self.height - 1 {
            for x_pos in 1..self.width - 1 {
                let choice = match rng.random_range(0..100) {
                    r if r < 60 => MapObject::Empty,
                    r if r < 80 => MapObject::Boulder(Boulder { x: 0, y: 0 }),
                    r if r < 85 => MapObject::Diamond(Diamond {}),
                    r if r < 95 => MapObject::Wall,
                    _ => MapObject::Dirt,
                };
                self.set(x_pos, y_pos, choice);
            }
        }
        /*
        for y_pos in 1..self.height - 1 {
            self.set(1, y_pos, MapObject::Boulder(Boulder { x: 0, y: 0 }));
            self.set(2, y_pos, MapObject::Empty);
            self.set(3, y_pos, MapObject::Empty);
            self.set(4, y_pos, MapObject::Empty);
            self.set(5, y_pos, MapObject::Empty);
        }
        */
        self.set(self.width / 2, self.height / 2, MapObject::Player);
    }

    pub fn update(&mut self) {
        self.clear_update();

        for y_pos in 1..self.height - 1 {
            for x_pos in 1..self.width - 1 {
                let offset = (y_pos as usize) * (self.width as usize) + (x_pos as usize);
                if self.updated[offset] {
                    continue;
                }
                let cell = self.get(x_pos, y_pos);
                match cell {
                    MapObject::Boulder(cell) => cell.update(self, x_pos, y_pos),
                    MapObject::Diamond(cell) => cell.update(self, x_pos, y_pos),
                    _ => {}
                };
            }
        }
    }

    pub fn set(&mut self, x_pos: u8, y_pos: u8, value: MapObject) {
        if x_pos >= self.width || y_pos >= self.height {
            return;
        }

        let offset = (y_pos as usize) * (self.width as usize) + (x_pos as usize);
        self.map[offset] = value;
    }

    fn clear_update(&mut self) {
        let map_size = (self.width as usize) * (self.height as usize);
        self.updated = vec![false; map_size].into_boxed_slice();
    }

    pub fn set_updated(&mut self, x_pos: u8, y_pos: u8) {
        let offset = (y_pos as usize) * (self.width as usize) + (x_pos as usize);
        self.updated[offset] = true;
    }

    pub fn get(&self, x_pos: u8, y_pos: u8) -> MapObject {
        if x_pos >= self.width || y_pos >= self.height {
            panic!("Out of bound in map");
        }

        let offset = (y_pos as usize) * (self.width as usize) + (x_pos as usize);
        self.map[offset]
    }

    pub fn get_empty(&self, x_pos: u8, y_pos: u8) -> bool {
        matches!(self.get(x_pos, y_pos), MapObject::Empty)
    }
}

impl fmt::Display for MapMatrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y_pos in 0..self.height {
            for x_pos in 0..self.width {
                let offset = (y_pos as usize) * (self.width as usize) + (x_pos as usize);
                let val = &self.map[offset];
                write!(f, "{}", val)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl fmt::Display for MapObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = match self {
            MapObject::Empty => ' ',
            MapObject::Wall => '█',
            MapObject::Boulder(_) => 'O',
            MapObject::Diamond(_) => '◆',
            MapObject::Dirt => '▒',
            MapObject::Player => '☻',
        };
        write!(f, "{}", val)?;
        Ok(())
    }
}
