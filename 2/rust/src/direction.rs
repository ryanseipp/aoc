use std::{error::Error, str::FromStr};

#[derive(Debug, PartialEq)]
pub enum Direction {
    UP(i32),
    DOWN(i32),
    FORWARD(i32),
}

impl FromStr for Direction {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();
        if parts.len() != 2 {
            return Err("Expected a direction and amount".into());
        }

        let amount = i32::from_str(parts[1])?;
        return match parts[0] {
            "up" => Ok(Direction::UP(amount)),
            "down" => Ok(Direction::DOWN(amount)),
            "forward" => Ok(Direction::FORWARD(amount)),
            x => Err(format!("Unknown direction: {}", x).into()),
        };
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use super::Direction;

    #[test]
    fn direction_parses_forward_command() {
        let command = "forward 1";
        let direction = Direction::from_str(command).expect("Could not parse");

        assert_eq!(direction, Direction::FORWARD(1));
    }

    #[test]
    fn direction_parses_up_command() {
        let command = "up 1";
        let direction = Direction::from_str(command).expect("Could not parse");

        assert_eq!(direction, Direction::UP(1));
    }

    #[test]
    fn direction_parses_down_command() {
        let command = "down 1";
        let direction = Direction::from_str(command).expect("Could not parse");

        assert_eq!(direction, Direction::DOWN(1));
    }

    #[test]
    fn error_result_on_invalid_direction() {
        let command = "top 1";
        let direction = Direction::from_str(command);

        assert!(direction.is_err());
    }
}
