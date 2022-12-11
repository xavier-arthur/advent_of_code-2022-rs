use std::ops::RangeInclusive;

pub fn get_array(pair: &Vec<&str>) -> Vec<RangeInclusive<i32>> {
    let mut pair_range: Vec<RangeInclusive<i32>> = Vec::new();

    for val in pair {
        let interval: Vec<i32> = val.split("-").map(|v| v.parse::<i32>().unwrap()).collect();
        pair_range.push(interval[0]..=interval[1]);
    }
    pair_range
}   

pub fn pair_incoporates(pair: &Vec<RangeInclusive<i32>>) -> bool {
    let mut pair_range: Vec<Vec<i32>> = Vec::new();

    for interval in pair {
        pair_range.push(
            interval.to_owned().collect::<Vec<i32>>()
        );
    }

    // sort pair range so the biggest in the first index
    pair_range.sort_by(|a, b| b.len().partial_cmp(&a.len()).unwrap());

    // the bigger contains all from the smallest?
    pair_range[1].iter().all(|f| pair_range[0].contains(f))
}