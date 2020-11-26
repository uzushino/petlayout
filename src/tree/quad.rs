
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

pub struct Tree<T> {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32, 
   
    pub nodes: Vec<T>,    

    children: Vec<Tree<T>>,
}

impl<T> Tree<T> {
    pub fn new() -> Tree<T> {
        Tree {
            x: 0f32,
            y: 0f32,
            width: 0f32,
            height: 0f32,

            nodes: Vec::default() as Vec<T>,
            children: Vec::with_capacity(4),
        }
    }

    pub fn blank_child(&mut self, dir: DIR, x: f32, y: f32, w: f32, h: f32) {
        self.children[dir.direction()] = Tree {
            x, 
            y, 
            width: w,
            height: h,
            nodes: Vec::default(),
            children: Vec::with_capacity(4),
        };
    }

    pub fn add_child(&mut self) {
    }
}
