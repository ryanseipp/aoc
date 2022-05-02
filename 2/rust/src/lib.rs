use std::str::FromStr;

use direction::Direction;
use position::Position;

pub mod direction;
pub mod position;

pub fn parse_directions(directions: &Vec<String>) -> Vec<Direction> {
    directions
        .iter()
        .map(|line| Direction::from_str(line).expect("Error parsing direction"))
        .collect()
}

pub fn determine_position(directions: &Vec<Direction>) -> i32 {
    let mut position = Position::new();

    directions
        .iter()
        .for_each(|direction| position.move_direction(direction));

    return position.position();
}

pub fn determine_position_with_aim_tracking(directions: &Vec<Direction>) -> i32 {
    let mut position = Position::new_with_aim_tracking();

    directions
        .iter()
        .for_each(|direction| position.move_direction(direction));

    return position.position();
}

#[cfg(test)]
mod test {
    use crate::{direction::Direction, parse_directions, determine_position, determine_position_with_aim_tracking};

    #[test]
    fn parse_directions_returns_correct_result() {
        let directions = vec![
            String::from("forward 5"),
            String::from("down 5"),
            String::from("forward 8"),
            String::from("up 3"),
            String::from("down 8"),
            String::from("forward 2"),
        ];

        let expected = vec![
            Direction::FORWARD(5),
            Direction::DOWN(5),
            Direction::FORWARD(8),
            Direction::UP(3),
            Direction::DOWN(8),
            Direction::FORWARD(2),
        ];

        let parsed = parse_directions(&directions);

        assert_eq!(parsed, expected);
    }

    #[test]
    fn determine_position_returns_correct_result() {
        let directions = vec![
            Direction::FORWARD(5),
            Direction::DOWN(5),
            Direction::FORWARD(8),
            Direction::UP(3),
            Direction::DOWN(8),
            Direction::FORWARD(2),
        ];

        let position = determine_position(&directions);

        assert_eq!(position, 150);
    }

    #[test]
    fn determine_position_with_aim_tracking_returns_correct_result() {
        let directions = vec![
            Direction::FORWARD(5),
            Direction::DOWN(5),
            Direction::FORWARD(8),
            Direction::UP(3),
            Direction::DOWN(8),
            Direction::FORWARD(2),
        ];

        let position = determine_position_with_aim_tracking(directions);

        assert_eq!(position, 900);
    }
}
