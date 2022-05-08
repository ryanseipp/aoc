use bitvec::array::BitArray;
use bitvec::bitarr;
use bitvec::order::Msb0;
use bitvec::view::BitViewSized;

pub fn parse_diagnostics(diagnostics: &Vec<String>) -> Vec<BitArray<u16, Msb0>> {
    return diagnostics
        .iter()
        .map(|diagnostic| u16::from_str_radix(&diagnostic, 2).unwrap().into_bitarray())
        .collect();
}

pub fn get_power_consumption(diagnostics: &Vec<BitArray<u16, Msb0>>) -> u32 {
    let frequency_thresh: usize = diagnostics.len() / 2;
    let mut frequency: [usize; 16] = [0; 16];
    for diagnostic in diagnostics {
        for (i, bit) in diagnostic.as_bitslice().iter().by_vals().enumerate() {
            if bit {
                frequency[i] += 1;
            }
        }
    }

    let mut gamma: BitArray<[u16; 1], Msb0> = bitarr![u16, Msb0; 0; 16];
    let mut epsilon: BitArray<[u16; 1], Msb0> = bitarr![u16, Msb0; 0; 16];

    for i in 0..16 {
        if frequency[i] >= frequency_thresh {
            gamma.set(i, true);
        } else if frequency[i] != 0 {
            epsilon.set(i, true);
        }
    }

    let gamma_dec: u32 = gamma.as_raw_slice()[0].into();
    let epsilon_dec: u32 = epsilon.as_raw_slice()[0].into();

    return gamma_dec * epsilon_dec;
}

pub fn get_life_support(bitlength: u8, diagnostics: &Vec<BitArray<u16, Msb0>>) -> u32 {
    let mut most_common = get_most_common_for_index(16 - bitlength, &diagnostics);
    let mut least_common = get_least_common_for_index(16 - bitlength, &diagnostics);

    for bit_position in (16 - bitlength + 1)..16 {
        if most_common.len() > 1 {
            most_common = get_most_common_for_index(bit_position, &most_common);
        }
        if least_common.len() > 1 {
            least_common = get_least_common_for_index(bit_position, &least_common);
        }
    }

    let oxygen_rating: u32 = most_common[0].as_raw_slice()[0].into();
    let co2_rating: u32 = least_common[0].as_raw_slice()[0].into();

    return oxygen_rating * co2_rating;
}

fn get_most_common_for_index(
    index: u8,
    diagnostics: &Vec<BitArray<u16, Msb0>>,
) -> Vec<BitArray<u16, Msb0>> {
    let frequency = get_frequency_for_index(index, &diagnostics);
    let one_most_common: bool = frequency >= (diagnostics.len() as f64) / 2.0;

    return diagnostics
        .iter()
        .filter(|&diag| diag.get(index as usize).as_deref() == Some(&one_most_common))
        .cloned()
        .collect();
}

fn get_least_common_for_index(
    index: u8,
    diagnostics: &Vec<BitArray<u16, Msb0>>,
) -> Vec<BitArray<u16, Msb0>> {
    let frequency = get_frequency_for_index(index, &diagnostics);
    let one_least_common: bool = frequency < (diagnostics.len() as f64) / 2.0;

    return diagnostics
        .iter()
        .filter(|&diag| diag.get(index as usize).as_deref() == Some(&one_least_common))
        .cloned()
        .collect();
}

fn get_frequency_for_index(index: u8, diagnostics: &Vec<BitArray<u16, Msb0>>) -> f64 {
    return diagnostics
        .iter()
        .map(|diag| -> u32 { diag.as_bitslice()[index as usize].into() })
        .reduce(|accum, item| accum + item)
        .expect("Expected result")
        .into();
}

#[cfg(test)]
mod test {
    use bitvec::bitarr;
    use bitvec::order::Msb0;

    use crate::{get_life_support, get_power_consumption, parse_diagnostics};

    #[test]
    fn parse_diagnostics_creates_correct_bitarray() {
        let diagnostics = vec![String::from("0101010101010101")];
        let expected = bitarr![u16, Msb0; 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1];

        let parsed = parse_diagnostics(&diagnostics);

        assert_eq!(parsed[0], expected);
    }

    #[test]
    fn get_power_consumption_produces_correct_result() {
        let diagnostics = vec![
            String::from("00100"),
            String::from("11110"),
            String::from("10110"),
            String::from("10111"),
            String::from("10101"),
            String::from("01111"),
            String::from("00111"),
            String::from("11100"),
            String::from("10000"),
            String::from("11001"),
            String::from("00010"),
            String::from("01010"),
        ];

        let parsed = parse_diagnostics(&diagnostics);
        let power = get_power_consumption(&parsed);

        assert_eq!(power, 198);
    }

    #[test]
    fn get_life_support_produces_correct_result() {
        let diagnostics = vec![
            String::from("00100"),
            String::from("11110"),
            String::from("10110"),
            String::from("10111"),
            String::from("10101"),
            String::from("01111"),
            String::from("00111"),
            String::from("11100"),
            String::from("10000"),
            String::from("11001"),
            String::from("00010"),
            String::from("01010"),
        ];

        let parsed = parse_diagnostics(&diagnostics);
        let life_support = get_life_support(5, &parsed);

        assert_eq!(life_support, 230);
    }
}
