
pub enum Position {
    NE, NW, SE, SW,
}

impl Position {
    pub fn direction(self) -> usize {
        match self {
            Position::NE => 0,
            Position::NW => 1,
            Position::SE => 2,
            Position::SW => 3,
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

    pub fn blank_child(&mut self, dir: Position, x: f32, y: f32, w: f32, h: f32) {
        self.children[dir.direction()] = Tree {
            x, 
            y, 
            width: w,
            height: h,
            nodes: Vec::default(),
            children: Vec::with_capacity(4),
        };
    }

    pub fn add_child(&mut self, value: &Tree<T>) {
    }

    fn is_contain(self, pos: Position, x: f32, y: f32) -> bool {
        match pos {
            Position::NW => self.is_nw(x, y),
            _ => false,
        }
    }

    fn is_nw(self, x: f32, y: f32) -> bool {
        x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height / 2.0
    }
}
