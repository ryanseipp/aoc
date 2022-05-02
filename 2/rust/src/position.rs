use crate::direction::Direction;

pub struct Position {
    track_aim: bool,
    x: i32,
    depth: i32,
    aim: i32,
}

impl Position {
    pub fn new() -> Position {
        return Position { track_aim: false, x: 0, depth: 0, aim: 0 };
    }

    pub fn new_with_aim_tracking() -> Position {
        let mut position = Position::new();
        position.track_aim = true;
        return position;
    }

    pub fn move_direction(&mut self, direction: &Direction) {
        if self.track_aim {
            return match direction {
                Direction::UP(aim) => self.aim -= aim,
                Direction::DOWN(aim) => self.aim += aim,
                Direction::FORWARD(x) => {
                    self.x += x;
                    self.depth += self.aim * x
                }
            }
        }

        return match direction {
            Direction::UP(depth) => self.depth -= depth,
            Direction::DOWN(depth) => self.depth += depth,
            Direction::FORWARD(x) => self.x += x,
        };
    }

    pub fn position(&self) -> i32 {
        return self.x * self.depth;
    }
}

#[cfg(test)]
mod test {
    use crate::direction::Direction;

    use super::Position;

    #[test]
    fn position_moves_up() {
        let mut position = Position::new();
        let direction = Direction::UP(1);

        position.move_direction(&direction);

        assert_eq!(position.x, 0);
        assert_eq!(position.depth, -1);
    }

    #[test]
    fn position_moves_down() {
        let mut position = Position::new();
        let direction = Direction::DOWN(1);

        position.move_direction(&direction);

        assert_eq!(position.x, 0);
        assert_eq!(position.depth, 1);
    }

    #[test]
    fn position_moves_forward() {
        let mut position = Position::new();
        let direction = Direction::FORWARD(1);

        position.move_direction(&direction);

        assert_eq!(position.x, 1);
        assert_eq!(position.depth, 0);
    }

    #[test]
    fn position_position_multiplies_x_y() {
        let mut position = Position::new();
        position.x = 10;
        position.depth = 5;
        assert_eq!(position.position(), 50);
    }

    #[test]
    fn track_aim_multiplies_aim_and_adds_to_position_on_forward_direction() {
        let mut position = Position::new_with_aim_tracking();
        position.aim = 5;

        position.move_direction(&Direction::FORWARD(8));
        
        assert_eq!(position.depth, 40);
    }
}
