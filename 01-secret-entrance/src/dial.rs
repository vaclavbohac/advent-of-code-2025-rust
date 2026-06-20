use crate::rotation::{Direction, parse_rotation};

#[derive(Debug, PartialEq)]
pub struct Dial {
    position: u32,
    zeros_crossed: u32,
}

impl Dial {
    pub(crate) fn new(position: u32) -> Dial {
        Dial {
            position,
            zeros_crossed: 0,
        }
    }

    pub fn rotate(&self, rotation: &str) -> Dial {
        let rotation_move = parse_rotation(rotation);

        let pos = self.position as i32;

        let new_pos = match rotation_move.direction() {
            Direction::Right => pos + rotation_move.diff(),
            Direction::Left => pos - rotation_move.diff(),
        };

        let zeros_crossed: u32 = match rotation_move.direction() {
            Direction::Right => ((pos + rotation_move.diff()) / 100) as u32,
            Direction::Left => {
                let first_hit = if pos == 0 { 100i32 } else { pos };

                if rotation_move.diff() < first_hit {
                    0
                } else {
                    (((rotation_move.diff() - first_hit) / 100) + 1) as u32
                }
            }
        };

        Dial {
            position: new_pos.rem_euclid(100) as u32,
            zeros_crossed,
        }
    }

    pub fn pointing_at_zero(&self) -> bool {
        self.position == 0
    }

    pub fn zeros_crossed(&self) -> u32 {
        self.zeros_crossed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_rotates_right() {
        assert_eq!(Dial::new(1), Dial::new(0).rotate("R1"));
        assert_eq!(Dial::new(2), Dial::new(1).rotate("R1"));
    }

    #[test]
    fn it_rotates_right_overflow() {
        assert_eq!(50, Dial::new(50).rotate("R100").position);
    }

    #[test]
    fn it_rotates_left() {
        assert_eq!(99, Dial::new(0).rotate("L1").position);
        assert_eq!(0, Dial::new(1).rotate("L1").position);
    }

    #[test]
    fn it_rotates_left_overflow() {
        assert_eq!(50, Dial::new(50).rotate("L200").position);
    }

    #[test]
    fn it_returns_true_if_position_is_zero() {
        assert!(Dial::new(0).pointing_at_zero());
        assert!(!Dial::new(1).pointing_at_zero());
    }

    #[test]
    fn it_returns_times_the_rotation_crossed_zero_right_rotation() {
        assert_eq!(1, Dial::new(0).rotate("R101").zeros_crossed);
        assert_eq!(1, Dial::new(50).rotate("R100").zeros_crossed);
        assert_eq!(1, Dial::new(99).rotate("R1").zeros_crossed);
    }

    #[test]
    fn it_returns_times_the_rotation_crossed_zero_left_rotation_simple() {
        assert_eq!(1, Dial::new(50).rotate("L68").zeros_crossed);
    }

    #[test]
    fn it_returns_times_the_rotation_crossed_zero_left_rotation() {
        assert_eq!(1, Dial::new(1).rotate("L1").zeros_crossed);
        assert_eq!(1, Dial::new(1).rotate("L2").zeros_crossed);
        assert_eq!(2, Dial::new(50).rotate("L150").zeros_crossed);
    }
}
