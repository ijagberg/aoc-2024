use std::ops::{Add, Mul};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: i64,
    pub y: i64,
}

impl Vec2 {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

impl Mul<i64> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: i64) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

pub struct ArcadeGame {
    target: Vec2,
    a: Vec2,
    b: Vec2,
}

impl ArcadeGame {
    pub fn new(target: Vec2, a: Vec2, b: Vec2) -> Self {
        Self { target, a, b }
    }

    pub fn win(&self) -> Option<(u64, u64)> {
        let det = self.a.x * self.b.y - self.b.x * self.a.y;
        if det == 0 {
            return None;
        }

        let det_n = self.target.x * self.b.y - self.b.x * self.target.y;
        let det_m = self.a.x * self.target.y - self.target.x * self.a.y;

        if det_n % det == 0 && det_m % det == 0 {
            // integer solution exists
            let n = det_n / det;
            let m = det_m / det;
            if n >= 0 && m >= 0 {
                return Some((n as u64, m as u64));
            }
        }

        None
    }

    pub fn target_mut(&mut self) -> &mut Vec2 {
        &mut self.target
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn win_test() {
        let arcade_game =
            ArcadeGame::new(Vec2::new(8400, 5400), Vec2::new(94, 34), Vec2::new(22, 67));

        assert_eq!(arcade_game.win(), Some((80, 40)));

        let arcade_game = ArcadeGame::new(
            Vec2::new(12748, 12176),
            Vec2::new(26, 66),
            Vec2::new(67, 21),
        );
        assert_eq!(arcade_game.win(), None);
    }
}
