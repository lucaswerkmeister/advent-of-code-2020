use std::collections::HashMap;
use std::env;
use std::fs;

fn part2(filename: &str) -> u64 {
    let input = fs::read_to_string(filename).expect("read input file");
    let line2 = input.lines().skip(1).next().expect("get second line");
    let mut departures_by_offset = HashMap::new();
    let mut offsets_by_departure = HashMap::new();
    for (offset, departure_str) in line2.split(",").enumerate() {
        let offset = offset as u64;
        if let Ok(departure) = departure_str.parse::<u64>() {
            departures_by_offset.insert(offset, departure);
            offsets_by_departure.insert(departure, offset);
        }
    }

    let mut departures_descending: Vec<u64> = departures_by_offset.values().map(|&i| i).collect();
    departures_descending.sort_unstable();
    let last_departure = departures_descending.pop().expect("at least one departure");
    let last_departure_offset = offsets_by_departure[&last_departure];
    departures_descending.reverse();
    'outer: for i in 0.. {
        let t = i * last_departure + last_departure - last_departure_offset;
        for &departure in departures_descending.iter() {
            let offset = offsets_by_departure[&departure];
            if (t + offset) % departure != 0 {
                continue 'outer;
            }
        }
        return t;
    }
    panic!("no solution found");
}

fn main() {
    println!(
        "{}",
        part2(&env::args().skip(1).next().expect("one argument"))
    );
}
