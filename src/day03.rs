use std::fs::File;
use std::io::prelude::*;

type Bit2DArray = Vec<Vec<u32>>;

pub fn day_03() -> std::io::Result<()> {
    //let mut file = File::open("./resources/day03_small.txt")?;
    let mut file = File::open("./resources/day03.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    println!("contents:\n{}", contents);

    let bit_2d_array: Bit2DArray = contents
        .trim()
        .split('\n')
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_digit(2u32).unwrap())
                .collect()
        })
        .collect();

    let col_size = bit_2d_array[0].len();
    //println!(" parse content: {:?}", bit_2d_array);
    println!(" parse content: {:?}", bit_2d_array[0]);

    let gamma_rate = (0..col_size)
        .map(|col| most_common_bit(&bit_2d_array, col))
        .collect::<Vec<u32>>();
    let epsilon_rate = (0..col_size)
        .map(|col| least_common_bit(&bit_2d_array, col))
        .collect::<Vec<u32>>();

    println!("gamma_rate {:?}", gamma_rate);
    println!("epsilon_rate {:?}", epsilon_rate);

    let gamma_rate = u32::from_str_radix(convert_into_binary_str(gamma_rate).as_str(), 2).unwrap();
    let epsilon_rate =
        u32::from_str_radix(convert_into_binary_str(epsilon_rate).as_str(), 2).unwrap();
    println!("Power consumption (part 1): {}", gamma_rate * epsilon_rate);

    let mut filtered_most_common = bit_2d_array.clone();
    let mut filtered_least_common = bit_2d_array.clone();

    for col in 0..col_size {
        let most_common = most_common_bit(&filtered_most_common, col);
        filtered_most_common =
            filter_array_with_bit_at_col(&filtered_most_common, most_common, col);
        if filtered_most_common.len() == 1 {
            break;
        }
    }
    for col in 0..col_size {
        let least_common = least_common_bit(&filtered_least_common, col);
        filtered_least_common =
            filter_array_with_bit_at_col(&filtered_least_common, least_common, col);
        if filtered_least_common.len() == 1 {
            break;
        }
    }

    println!("filtered_most_common: {:?}", filtered_most_common);
    let oxygen_gen_rate = u32::from_str_radix(
        convert_into_binary_str(filtered_most_common[0].clone()).as_str(),
        2,
    )
    .unwrap();
    println!("oxygen_gen_rate: {:?}", oxygen_gen_rate);
    let co2_rate = u32::from_str_radix(
        convert_into_binary_str(filtered_least_common[0].clone()).as_str(),
        2,
    )
    .unwrap();
    println!("co2_rate: {:?}", co2_rate);
    println!(
        "Life support rating (part 2): {}",
        oxygen_gen_rate * co2_rate
    );

    Ok(())
}

fn most_common_bit(bit_2d_array: &Bit2DArray, col: usize) -> u32 {
    let bit0_count = bit_2d_array.iter().filter(|row| row[col] == 0).count();
    let bit1_count = bit_2d_array.iter().filter(|row| row[col] == 1).count();
    if bit0_count > bit1_count {
        return 0;
    }
    return 1;
}

fn least_common_bit(bit_2d_array: &Bit2DArray, col: usize) -> u32 {
    if most_common_bit(bit_2d_array, col) == 0 {
        1
    } else {
        0
    }
}

fn convert_into_binary_str(v: Vec<u32>) -> String {
    v.iter()
        .map(|d| if *d == 0u32 { '0' } else { '1' })
        .collect()
}

fn filter_array_with_bit_at_col(bit_2d_array: &Bit2DArray, bit: u32, col: usize) -> Bit2DArray {
    let new_2d_array: Bit2DArray = bit_2d_array
        .iter()
        .filter(|row| row[col] == bit)
        .map(|row| row.clone()) // TODO: how to avoid clone?
        .collect();
    return new_2d_array;
}
