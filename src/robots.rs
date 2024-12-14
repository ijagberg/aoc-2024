use std::{
    collections::{HashMap, HashSet},
    ops::AddAssign,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Robots {
    width: u64,
    height: u64,
    robots: Vec<Robot>,
}

impl Robots {
    pub fn new(width: u64, height: u64, robots: Vec<Robot>) -> Self {
        assert!(width % 2 == 1);
        assert!(height % 2 == 1);
        Self {
            width,
            height,
            robots,
        }
    }

    pub fn is_horizontally_symmetrical(&self) -> bool {
        let mut coord_and_count = HashMap::new();

        for robot in &self.robots {
            if let Some(_) = robot.quadrant(self.width, self.height) {
                // only look at robots that should have a reflection
                coord_and_count
                    .entry((robot.xpos, robot.ypos))
                    .or_insert(0_u64)
                    .add_assign(1);
            }
        }

        for (&(x, y), &count) in &coord_and_count {
            if x < self.width / 2 {
                let reflected = self.width - x - 1;
                if coord_and_count.get(&(reflected, y)).copied() != Some(count) {
                    return false;
                }
            }
        }

        true
    }

    pub fn count_in_quadrants(&self) -> (u64, u64, u64, u64) {
        let (mut top_left, mut top_right, mut bottom_right, mut bottom_left) = (0, 0, 0, 0);
        for robot in &self.robots {
            if let Some((left, top)) = robot.quadrant(self.width, self.height) {
                match (left, top) {
                    (true, true) => top_left += 1,
                    (true, false) => bottom_left += 1,
                    (false, true) => top_right += 1,
                    (false, false) => bottom_right += 1,
                }
            }
        }
        (top_left, top_right, bottom_right, bottom_left)
    }

    pub fn robots(&self) -> &Vec<Robot> {
        &self.robots
    }

    pub fn run(&mut self, seconds: u64) {
        for robot in &mut self.robots {
            let next_x: i64 = (robot.xpos as i64 + seconds as i64 * robot.vx) % self.width as i64;
            if next_x.is_negative() {
                robot.xpos = (self.width as i64 + next_x) as u64
            } else {
                robot.xpos = next_x as u64;
            }
            let mut next_y: i64 =
                (robot.ypos as i64 + seconds as i64 * robot.vy) % self.height as i64;
            if next_y.is_negative() {
                robot.ypos = (self.height as i64 + next_y) as u64;
            } else {
                robot.ypos = next_y as u64;
            }
        }
    }

    pub fn width(&self) -> u64 {
        self.width
    }

    pub fn height(&self) -> u64 {
        self.height
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Robot {
    vx: i64,
    vy: i64,
    xpos: u64,
    ypos: u64,
}

impl Robot {
    pub fn new(vx: i64, vy: i64, xpos: u64, ypos: u64) -> Self {
        Self { vx, vy, xpos, ypos }
    }

    fn quadrant(&self, width: u64, height: u64) -> Option<(bool, bool)> {
        if self.xpos == width / 2 || self.ypos == height / 2 {
            None
        } else {
            let is_left = self.xpos < width / 2;
            let is_top = self.ypos < height / 2;
            Some((is_left, is_top))
        }
    }

    pub fn vx(&self) -> i64 {
        self.vx
    }

    pub fn vy(&self) -> i64 {
        self.vy
    }

    pub fn xpos(&self) -> u64 {
        self.xpos
    }

    pub fn ypos(&self) -> u64 {
        self.ypos
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn modulo_test() {
        assert_eq!(-1 % 100, -1);
        assert_eq!(-32 % 11, 1);
    }
}
