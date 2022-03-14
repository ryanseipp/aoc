pub fn count_larger_measurements(depths: &Vec<i32>) -> u32 {
    depths
        .iter()
        .zip(depths.iter().skip(1))
        .fold(0, |acc, x| match x.1 > x.0 {
            true => acc + 1,
            false => acc,
        })
}

pub fn count_three_larger(depths: &Vec<i32>) -> u32 {
    let summed: Vec<i32> = depths
        .iter()
        .zip(depths.iter().skip(1))
        .zip(depths.iter().skip(2))
        .map(|x| x.0 .0 + x.0 .1 + x.1)
        .collect();

    count_larger_measurements(&summed)
}

#[cfg(test)]
mod test {
    use crate::{count_larger_measurements, count_three_larger};

    #[test]
    fn count_larger_measurements_returns_correct_result() {
        let depths = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        let result = count_larger_measurements(&depths);

        assert_eq!(result, 7);
    }

    #[test]
    fn count_larger_threes_returns_correct_result() {
        let depths = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        let result = count_three_larger(&depths);

        assert_eq!(result, 5);
    }
}
