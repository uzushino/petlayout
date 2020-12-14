
pub enum Position {
    NE, NW, SE, SW, None
}

impl Position {
    pub fn direction(self) -> usize {
        match self {
            Position::NE => 0,
            Position::NW => 1,
            Position::SE => 2,
            Position::SW => 3,
            Position::None => 4,
        }
    }
}

pub struct Tree {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32, 
   
    pub nodes: Vec<Tree>,

    children: Option<Vec<Tree>>,
}

impl Tree {
    pub fn new() -> Tree {
        Tree {
            x: 0f32,
            y: 0f32,
            width: 0f32,
            height: 0f32,

            nodes: Vec::default(),
            children: None,
        }
    }

    pub fn new_with_divide(other: Tree, pos: Position) -> Tree {
        let (x, y, width, height) = match pos {
            Position::NW => (other.x, other.y, other.width / 2.0, other.height / 2.0),
            Position::NE => (other.x + other.width / 2.0, other.y, other.width, other.height / 2.0),
            Position::SW => (other.x, other.y + other.height / 2.0, other.width / 2.0, other.height),
            Position::SE => (other.x + other.width / 2.0, other.y + other.height / 2.0, other.width, other.height),
        };
        
        Tree {
            x,
            y,
            width,
            height,

            nodes: Vec::default(),
            children: None,
        }
    }

    pub fn add_child(&mut self, value: Tree) {
        match (self.is_contain(value.x, value.y), &mut self.children) {
            (Position::NW, Some(ref mut children)) => children[Position::NW.direction()].add_child(value),
            (Position::NE, Some(ref mut children)) => children[Position::NE.direction()].add_child(value),
            (Position::SW, Some(ref mut children)) => children[Position::SW.direction()].add_child(value),
            (Position::SE, Some(ref mut children)) => children[Position::SE.direction()].add_child(value),
            _ => self.nodes.push(value),
        }
    }

    pub fn divide_area(&mut self) {
        if self.children.is_some() { // already divided
            return ;
        }

        self.children = Some(
            vec![
                Tree::
            ]
        )
    }

    fn is_contain(&self, x: f32, y: f32) -> Position {
        if self.is_nw(x, y) {
            Position::NW
        } else if self.is_nw(x, y) {
            Position::NE
        } else if self.is_nw(x, y) {
            Position::SW
        } else if self.is_nw(x, y) {
            Position::SE
        } else {
            Position::None
        }
    }

    fn is_nw(&self, x: f32, y: f32) -> bool {
        x >= self.x && x <= self.x + self.width / 2.0 && y >= self.y && y <= self.y + self.height / 2.0
    }
    
    fn is_ne(&self, x: f32, y: f32) -> bool {
        x >= self.x + self.width / 2.0 && x <= self.x + self.width && y >= self.y && y <= self.y + self.height / 2.0
    }
    
    fn is_sw(&self, x: f32, y: f32) -> bool {
        x >= self.x && x <= self.x + self.width / 2.0 && y >= self.y + self.height / 2.0 && y <= self.y
    }
    
    fn is_se(&self, x: f32, y: f32) -> bool {
        x >= self.x + self.width / 2.0 && x <= self.x + self.width && y >= self.y + self.height / 2.0 && y <= self.y + self.height
    }
}

mod test {
    use super::*;

    #[test]
    fn add_child() {
        let mut tree = Tree::new();
        tree.x = 0f32;
        tree.y = 0f32;
        tree.width = 10f32;
        tree.height = 10f32;

        let mut value = Tree::new();
        value.x = 1f32;
        value.y = 1f32;
        tree.width = 1f32;
        tree.height = 1f32;

        tree.add_child(value);

        assert!(tree.nodes.len() == 1)
    }
}
