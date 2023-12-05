use crate::utils::file_utils;

type SeedMap = Vec<Range>;

// TODO it would probably be way faster to assume an end value of 0 and then backtrack through 
// the maps to check if its one of the seeds. If not, increase the end value by 1 and keep doing
// that until we find a seed.
pub fn execute() {
    let lines = file_utils::get_all_lines("assets/input_5.txt");

    let seeds = parse_seeds(&lines[0]);
    let maps: Vec<Vec<Range>> = parse_maps(lines[2..].to_vec());

    let mut nr_seeds = 0;
    let mut i = 1;
    while i < seeds.len() {
        nr_seeds += seeds[i];
        i += 2;
    }

    let mut smallest_end_value = u64::MAX;

    i = 0;
    let mut seeds_checked = 0;
    while i < seeds.len() {
        for seed in seeds[i]..(seeds[i]+seeds[i + 1]) {
            let end_value = get_seed_end(seed, &maps);
            if end_value < smallest_end_value {
                smallest_end_value = end_value;
            }

            seeds_checked += 1;
            if seeds_checked % 1_000_000 == 0 {
                println!("Progress: {}%", seeds_checked * 100 / nr_seeds);
            }
        }
        i += 2;
    }

    println!("{smallest_end_value}");
}

#[derive(Debug, Clone)]
struct Range {
    destination: u64,
    source: u64,
    length: u64,
}

fn parse_seeds(input: &str) -> Vec<u64> {
    let seeds_string = input.split(":").collect::<Vec<_>>()[1];
    let mut seeds: Vec<u64> = Vec::new();

    for seed_to_parse in seeds_string.split(" ").collect::<Vec<_>>() {
        if seed_to_parse.is_empty() {
            continue;
        }

        seeds.push(seed_to_parse.parse().unwrap());
    }

    seeds
}

fn parse_range(input: &str) -> Range {
    let segments_to_filter = input.split(" ").collect::<Vec<_>>();
    let mut range_segments: Vec<&str> = Vec::new();

    for segment_to_filter in segments_to_filter {
        if segment_to_filter.is_empty() || segment_to_filter == " " {
            continue;
        }

        range_segments.push(segment_to_filter);
    }

    if range_segments.len() != 3 {
        panic!("failed to parse range from input {input}");
    }

    Range {
        destination: range_segments[0].parse().unwrap(),
        source: range_segments[1].parse().unwrap(),
        length: range_segments[2].parse().unwrap(),
    }
}

fn parse_maps(lines: Vec<String>) -> Vec<SeedMap> {
    let mut maps: Vec<SeedMap> = Default::default();
    let mut current_map: SeedMap = Default::default();

    for i in 2..lines.len() {
        if lines[i].is_empty() {
            continue;
        }

        if lines[i].contains(":") {
            maps.push(current_map.clone());
            current_map.clear();
        } else {
            current_map.push(parse_range(&lines[i]));
        }
    }

    maps.push(current_map);
    maps
}

fn get_seed_end(seed: u64, maps: &Vec<SeedMap>) -> u64 {
    let mut current_value = seed;

    for map in maps {
        for range in map {
            if current_value >= range.source && current_value < range.source + range.length {
                current_value += range.destination;
                current_value -= range.source;
                break;
            }
        }
    }

    current_value
}
