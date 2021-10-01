// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    position: (i32, i32),
    direction: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot {
            position: (x, y),
            direction: d,
        }
    }

    pub fn turn_right(self) -> Self {
        use Direction::*;

        match self.direction {
            North => Robot {
                position: self.position,
                direction: East,
            },
            East => Robot {
                position: self.position,
                direction: South,
            },
            South => Robot {
                position: self.position,
                direction: West,
            },
            West => Robot {
                position: self.position,
                direction: North,
            },
        }
    }

    pub fn turn_left(self) -> Self {
        use Direction::*;

        match self.direction {
            North => Robot {
                position: self.position,
                direction: West,
            },
            East => Robot {
                position: self.position,
                direction: North,
            },
            South => Robot {
                position: self.position,
                direction: East,
            },
            West => Robot {
                position: self.position,
                direction: South,
            },
        }
    }

    pub fn advance(self) -> Self {
        use Direction::*;

        match self.direction {
            North => Robot {
                position: (self.position.0, self.position.1 + 1),
                direction: self.direction,
            },
            East => Robot {
                position: (self.position.0 + 1, self.position.1),
                direction: self.direction,
            },
            South => Robot {
                position: (self.position.0, self.position.1 - 1),
                direction: self.direction,
            },
            West => Robot {
                position: (self.position.0 - 1, self.position.1),
                direction: self.direction,
            },
        }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        let mut ret = self;
        for c in instructions.chars() {
            match c {
                'R' => ret = ret.turn_right(),
                'L' => ret = ret.turn_left(),
                'A' => ret = ret.advance(),
                _ => {}
            }
        }
        ret
    }

    pub fn position(&self) -> (i32, i32) {
        self.position
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
