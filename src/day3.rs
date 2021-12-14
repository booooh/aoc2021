use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

enum BitCriteria {
    MostCommon,
    LeastCommon,
}

struct DiagnosticReport {
    binary_numbers: Vec<Vec<u8>>,
}

impl DiagnosticReport {
    fn num_bits(&self) -> usize {
        return self.binary_numbers[0].len();
    }
    fn bit_values(&self, bit_index: usize) -> Vec<u8> {
        self.binary_numbers
            .iter()
            .map(|number| number[bit_index])
            .collect()
    }

    fn most_common_bit(&self, bit_index: usize) -> u8 {
        let num_zeros = self
            .bit_values(bit_index)
            .iter()
            .filter(|b| **b == 0u8)
            .count();
        if num_zeros > self.binary_numbers.len() / 2 {
            0
        } else {
            1
        }
    }

    fn least_common_bit(&self, bit_index: usize) -> u8 {
        let mcb = self.most_common_bit(bit_index);
        if mcb == 0 {
            1
        } else {
            0
        }
    }

    fn gamma(&self) -> usize {
        let most_common_bits = (0..self.num_bits())
            .map(|bit_index| self.most_common_bit(bit_index))
            .collect::<Vec<_>>();
        return bit_vector_to_number(&most_common_bits);
    }

    fn epsilon(&self) -> usize {
        let least_common_bits = (0..self.num_bits())
            .map(|bit_index| self.least_common_bit(bit_index))
            .collect::<Vec<_>>();
        return bit_vector_to_number(&least_common_bits);
    }

    /**
         * Next, you should verify the life support rating, which can be determined by multiplying the oxygen generator rating by the CO2 scrubber rating.

    Both the oxygen generator rating and the CO2 scrubber rating are values that can be found in your diagnostic report - finding them is the tricky part.
    Both values are located using a similar process that involves filtering out values until only one remains.
    Before searching for either rating value, start with the full list of binary numbers from your diagnostic report and consider just the first bit of those numbers. Then:

    Keep only numbers selected by the bit criteria for the type of rating value for which you are searching. Discard numbers which do not match the bit criteria.
    If you only have one number left, stop; this is the rating value for which you are searching.
    Otherwise, repeat the process, considering the next bit to the right.
    The bit criteria depends on which type of rating value you want to find:

    To find oxygen generator rating, determine the most common value (0 or 1) in the current bit position, and keep only numbers with that bit in that position. If 0 and 1 are equally common, keep values with a 1 in the position being considered.
    To find CO2 scrubber rating, determine the least common value (0 or 1) in the current bit position, and keep only numbers with that bit in that position. If 0 and 1 are equally common, keep values with a 0 in the position being considered.
    For example, to determine the oxygen generator rating value using the same example diagnostic report from above:

    Start with all 12 numbers and consider only the first bit of each number. There are more 1 bits (7) than 0 bits (5), so keep only the 7 numbers with a 1 in the first position: 11110, 10110, 10111, 10101, 11100, 10000, and 11001.
    Then, consider the second bit of the 7 remaining numbers: there are more 0 bits (4) than 1 bits (3), so keep only the 4 numbers with a 0 in the second position: 10110, 10111, 10101, and 10000.
    In the third position, three of the four numbers have a 1, so keep those three: 10110, 10111, and 10101.
    In the fourth position, two of the three numbers have a 1, so keep those two: 10110 and 10111.
    In the fifth position, there are an equal number of 0 bits and 1 bits (one each). So, to find the oxygen generator rating, keep the number with a 1 in that position: 10111.
    As there is only one number left, stop; the oxygen generator rating is 10111, or 23 in decimal.
         */
    fn filter_bit_criteria(&self, bit_index: usize, criteria: BitCriteria) -> Self {
        // filters according to criteria, and returns a filtered report
        match criteria {
            BitCriteria::MostCommon => {
                let common_bit = self.most_common_bit(bit_index);
                filter_by_bit(self, bit_index, common_bit)
            }
            BitCriteria::LeastCommon => {
                let least_common_bit = self.least_common_bit(bit_index);
                filter_by_bit(self, bit_index, least_common_bit)
            }
        }
    }

    fn co2_scrubber_rating(&self) -> Vec<u8> {
        let mut filtered_report = DiagnosticReport {
            binary_numbers: self.binary_numbers.to_owned(),
        };
        for bit_index in 0..self.num_bits() {
            filtered_report =
                filtered_report.filter_bit_criteria(bit_index, BitCriteria::LeastCommon);
            if filtered_report.binary_numbers.len() == 1 {
                println!("found 1");
                break;
            }
        }
        return filtered_report.binary_numbers[0].to_owned();
    }

    fn oxygen_generator_rating(&self) -> Vec<u8> {
        let mut filtered_report = DiagnosticReport {
            binary_numbers: self.binary_numbers.to_owned(),
        };
        for bit_index in 0..self.num_bits() {
            filtered_report =
                filtered_report.filter_bit_criteria(bit_index, BitCriteria::MostCommon);
            if filtered_report.binary_numbers.len() == 1 {
                println!("found 1");
                break;
            }
            println! {"filtered down to {}", filtered_report.binary_numbers.len()};
        }
        return filtered_report.binary_numbers[0].to_owned();
    }
}

pub(crate) fn day3part2() -> (usize, usize) {
    let lines = read_lines("input3.t").unwrap();
    let binary_numbers: Vec<_> = lines.map(|l| parse_status_bits(l.unwrap())).collect();
    let diagnostics = DiagnosticReport { binary_numbers };
    return (
        bit_vector_to_number(&diagnostics.oxygen_generator_rating()),
        bit_vector_to_number(&diagnostics.co2_scrubber_rating()),
    );
}

fn filter_by_bit(report: &DiagnosticReport, bit_index: usize, bit_value: u8) -> DiagnosticReport {
    println!("starting with {}", report.binary_numbers.len());
    let filtered_values = report
        .binary_numbers
        .iter()
        .filter(|binary_number| binary_number[bit_index] == bit_value)
        .map(|x| x.to_owned())
        .collect::<Vec<Vec<u8>>>();
    println!("filtered to {}", filtered_values.len());
    return DiagnosticReport {
        binary_numbers: filtered_values,
    };
}

pub(crate) fn day3part1() -> (usize, usize) {
    let lines = read_lines("input3.t").unwrap();
    let binary_numbers: Vec<_> = lines.map(|l| parse_status_bits(l.unwrap())).collect();
    let diagnostics = DiagnosticReport { binary_numbers };
    return (diagnostics.gamma(), diagnostics.epsilon());
}

fn parse_status_bits(s: String) -> Vec<u8> {
    s.chars().map(|c| c.to_digit(2).unwrap() as u8).collect()
}

fn bit_vector_to_number(bit_vector: &Vec<u8>) -> usize {
    let string_bits = bit_vector
        .iter()
        .map(|bit| bit.to_string())
        .collect::<String>();
    let val = usize::from_str_radix(&string_bits, 2).unwrap();
    return val;
}
