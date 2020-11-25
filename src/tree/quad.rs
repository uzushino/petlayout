
pub enum DIR {
    NE, NW, SE, SW,
}

impl DIR {
    pub fn direction(self) -> usize {
        match self {
            DIR::NE => 0,
            DIR::NW => 1,
            DIR::SE => 2,
            DIR::SW => 3,
        }
    }
}

pub struct Tree {
    square: Rectangle,
    children: Vec<Tree>,
}

pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32, 
}

impl Rectangle {
    pub fn new() -> Self {
        Self {
            x: 0.0, 
            y: 0.0, 
            width: 0.0, 
            height: 0.0,
        }
    }
}

impl Tree {
    pub fn new() -> Self {
        Self {
            square: Rectangle::new(),
            children: Vec::with_capacity(4),
        }
    }

    pub fn add_child(&mut self, dir: DIR, x: f32, y: f32, w: f32, h: f32) {
        self.children[dir.direction()] = Tree {
            square: Rectangle {
                x, 
                y, 
                width: w,
                height: h,
            },
            children: Vec::with_capacity(4),
        };
    }
}
