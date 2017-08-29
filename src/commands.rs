use difflib::sequencematcher::SequenceMatcher;
use rayon::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::str;
use strsim::{osa_distance, damerau_levenshtein, jaro};
use utils;

fn get_ratio(s1: &str, s2: &str) -> io::Result<f32> {
    let mut one_two_matcher = SequenceMatcher::new(s1, s2);
    let mut two_one_matcher = SequenceMatcher::new(s2, s1);

    Ok(f32::max(one_two_matcher.ratio(), two_one_matcher.ratio()))
}

pub fn score(file1: &str, file2: &str) -> io::Result<()> {
    let s1 = utils::read_file_to_end(file1)?;
    let s2 = utils::read_file_to_end(file2)?;

    println!("{}", get_ratio(&s1, &s2)?);

    Ok(())
}

pub fn multiscore_one_list(file: &str, in_mem: bool) -> io::Result<()> {
    // Parse and validate the list of files
    let file_list: Vec<String>;
    {
        let f = File::open(file)?;
        let reader = BufReader::new(f);
        file_list = utils::parse_file_list(reader)?;
    }

    // Load all files into HashMap in memory, if specified
    let mut file_to_contents_map: HashMap<&String, String>;
    if in_mem {
        file_to_contents_map = HashMap::with_capacity(file_list.len());
        utils::file_list_to_hashmap(&file_list, &mut file_to_contents_map)?;
    } else {
        file_to_contents_map = HashMap::with_capacity(0);
    }

    // Calculate the similarity ratios in parallel
    let results: Vec<io::Result<(&String, &String, f32)>> = file_list.par_iter()
        .enumerate()
        .flat_map::<_, Vec<io::Result<(&String, &String, f32)>>>(|(i, file1)| {
            file_list[(i+1)..].par_iter().map(|file2| {
                let ratio = match in_mem {
                    true  => get_ratio(file_to_contents_map.get(file1).unwrap(), file_to_contents_map.get(file2).unwrap())?,
                    false => get_ratio(&utils::read_file_to_end(file1)?, &utils::read_file_to_end(file2)?)?,
                };

                Ok((file1, file2, ratio))
            }).collect()
    }).collect();

    // Check that all results were successful
    let mut ratios: Vec<(&String, &String, f32)> = Vec::with_capacity(results.len());
    for res in results {
        match res {
            Err(e) => return Err(e),
            Ok(v)  => ratios.push(v),
        }
    }

    // Print out the results in CSV form
    utils::print_results(&ratios)?;

    Ok(())
}

pub fn multiscore_two_lists(file1: &str, file2: &str, in_mem: bool) -> io::Result<()> {
    // Parse and validate the lists of files
    let file_list1: Vec<String>;
    {
        let f = File::open(file1)?;
        let reader = BufReader::new(f);
        file_list1 = utils::parse_file_list(reader)?;
    }
    
    let file_list2: Vec<String>;
    {
        let f = File::open(file2)?;
        let reader = BufReader::new(f);
        file_list2 = utils::parse_file_list(reader)?;
    }

    // Load all files into HashMap in memory, if specified
    let mut file_to_contents_map: HashMap<&String, String>;
    if in_mem {
        file_to_contents_map = HashMap::with_capacity(file_list1.len() + file_list2.len());
        utils::file_list_to_hashmap(&file_list1, &mut file_to_contents_map)?;
        utils::file_list_to_hashmap(&file_list2, &mut file_to_contents_map)?;
    } else {
        // Do not allocate
        file_to_contents_map = HashMap::with_capacity(0);
    }

    // Calculate the similarity ratios in parallel
    let results: Vec<io::Result<(&String, &String, f32)>> = file_list1.par_iter()
        .flat_map::<_, Vec<io::Result<(&String, &String, f32)>>>(|file1| {
            file_list2.par_iter().map(|file2| {
                let ratio = match in_mem {
                    true  => get_ratio(file_to_contents_map.get(file1).unwrap(), file_to_contents_map.get(file2).unwrap())?,
                    false => get_ratio(&utils::read_file_to_end(file1)?, &utils::read_file_to_end(file2)?)?,
                };

                Ok((file1, file2, ratio))
            }).collect()
    }).collect();

    // Check that all results were successful
    let mut ratios: Vec<(&String, &String, f32)> = Vec::with_capacity(results.len());
    for res in results {
        match res {
            Err(e) => return Err(e),
            Ok(v)  => ratios.push(v),
        }
    }

    // Print out the ratios in CSV form
    utils::print_results(&ratios)?;

    Ok(())
}
