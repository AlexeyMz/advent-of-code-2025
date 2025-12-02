//! Day 2: Gift Shop
use core::{get_data_path, MeasureElapsed};
use std::{fs::read_to_string, ops::RangeInclusive};

fn main() {
    let mut time = MeasureElapsed::start();
    basic().unwrap();
    time.print_measured("[basic]");
    advanced().unwrap();
    time.print_measured("[advanced]");
}

fn basic() -> Result<(), String> {
    let input = read_to_string(get_data_path("input/puzzle02.txt")).unwrap();

    let ranges = parse_ranges(&input)?;
    println!("Ranges: {:?}", ranges);

    let mut total = 0;
    for range in ranges {
        let even_ranges: Vec<RangeInclusive<i64>> = split_range_by_digit_count(range.clone())
            .into_iter()
            .filter(|r| digit_count(*r.start()) % 2 == 0)
            .collect();
        println!("Split {:?} into even ranges: {:?}", range, even_ranges);

        for sub_range in even_ranges {
            let (ah, al) = split_in_half(*sub_range.start());
            let (bh, bl) = split_in_half(*sub_range.end());

            if ah >= al && (ah < bh || ah <= bl) {
                let id = merge_from_halves(ah, ah);
                total += id;
                println!(" - invalid ID: {}", id);
            }
            if bh > ah && bl >= bh {
                let id = merge_from_halves(bh, bh);
                total += id;
                println!(" - invalid ID: {}", id);
            }
            for i in (ah + 1)..=(bh - 1) {
                let id = merge_from_halves(i, i);
                total += id;
                println!(" - invalid ID: {}", id);
            }
        }
    }

    println!("Total of invalid IDs: {total}");

    return Ok(());
}

fn advanced() -> Result<(), String> {
    let input = read_to_string(get_data_path("input/puzzle02.txt")).unwrap();

    let ranges = parse_ranges(&input)?;
    println!("Ranges: {:?}", ranges);

    let mut total = 0;
    for range in ranges {
        let sub_ranges: Vec<RangeInclusive<i64>> = split_range_by_digit_count(range.clone());
        println!("Split {:?} into sub ranges: {:?}", range, sub_ranges);

        for sub_range in sub_ranges {
            let digits = digit_count(*sub_range.start());
            for id in sub_range {
                if can_construct_from_duplicated_part(id, digits) {
                    total += id;
                    println!(" - invalid ID: {}", id);
                }
            }
        }
    }

    println!("Total of multi-duplicated invalid IDs: {total}");

    return Ok(());
}

fn parse_ranges(input: &str) -> Result<Vec<RangeInclusive<i64>>, String> {
    input.trim_end().split(",")
        .map(|range| {
            let boundaries: Vec<&str> = range.split("-").collect();
            if boundaries.len() == 2 {
                let first = boundaries[0].parse::<i64>();
                let second = boundaries[1].parse::<i64>();
                if let (Ok(first), Ok(second)) = (first, second) {
                    return Ok(first..=second);
                }
            }
            Err(format!("Invalid range: {range}"))
        })
        .collect()
}

fn split_range_by_digit_count(range: RangeInclusive<i64>) -> Vec<RangeInclusive<i64>> {
    let mut sub_ranges = vec![];
    let mut rest = range;
    let mut start_digits = digit_count(*rest.start());
    let end_digits = digit_count(*rest.end());
    while start_digits < end_digits {
        let start = *rest.start();
        let next_start = 10_i64.pow(start_digits);
        sub_ranges.push(start..=(next_start - 1));
        rest = next_start..=(*rest.end());
        start_digits += 1;
    }
    sub_ranges.push(rest);
    return sub_ranges;
}

fn digit_count(n: i64) -> u32 {
    if n < 0 {
        return 0;
    } else if n == 0 {
        return 1;
    }
    let mut x = n;
    let mut count = 0;
    while x > 0 {
        x /= 10;
        count += 1;
    }
    return count;
}

fn split_in_half(n: i64) -> (i64, i64) {
    let count = digit_count(n);
    let shift = 10_i64.pow(count / 2);
    (n / shift, n % shift)
}

fn merge_from_halves(high: i64, low: i64) -> i64 {
    let count = digit_count(low);
    let shift = 10_i64.pow(count);
    high * shift + low
}

fn can_construct_from_duplicated_part(n: i64, digits: u32) -> bool {
    'next_size: for part_size in 1..=(digits / 2) {
        if digits % part_size == 0 {
            let shift = 10_i64.pow(part_size);
            let low = n % shift;
            let mut rest = n / shift;
            while rest > 0 {
                if rest % shift == low {
                    rest /= shift;
                } else {
                    continue 'next_size;
                }
            }
            return  true;
        }
    }
    return false;
}
