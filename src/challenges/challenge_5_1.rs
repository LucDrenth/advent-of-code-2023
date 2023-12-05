use crate::utils::file_utils;

type SeedMap = Vec<Range>;

pub fn execute() {
    let lines = file_utils::get_all_lines("assets/input_5.txt");

    let seeds = parse_seeds(&lines[0]);
    let maps: Vec<Vec<Range>> = parse_maps(lines[2..].to_vec());

    let mut seed_ends: Vec<u64> = vec![];

    for seed in seeds {
        seed_ends.push( get_seed_end(seed, &maps) );
    }

    seed_ends.sort();

    println!("{:?}", seed_ends[0]);
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
