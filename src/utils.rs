use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::io;
use std::io::{BufRead, Read, Write};

pub fn read_file_to_end(file: &str) -> io::Result<String> {
    let mut contents = String::new();
    let mut f = File::open(file)?;
    f.read_to_string(&mut contents)?;

    Ok(contents)
}

pub fn parse_file_list<B>(lines_buf: B) -> io::Result<Vec<String>>
    where B: BufRead
{
    let lines = lines_buf.lines();
    let mut file_paths = Vec::with_capacity(lines.size_hint().0);

    // Check that all files exist
    for file_str in lines {
        let file_str = file_str?;
        if file_str.is_empty() {
            continue;
        } else if !Path::new(&file_str).exists() {
            return Err(io::Error::new(io::ErrorKind::NotFound, format!("The file '{}' was not found!", file_str)));
        }

        file_paths.push(file_str);
    }

    Ok(file_paths)
}

pub fn file_list_to_hashmap<'a>(file_list: &'a Vec<String>, map: &mut HashMap<&'a String, String>) -> io::Result<()> {
    for file_name in file_list {
        let contents = read_file_to_end(file_name)?;
        map.insert(file_name, contents);
    }

    Ok(())
}

pub fn print_results(results: &Vec<(&String, &String, f32)>) -> io::Result<()> {
    let stdout = io::stdout();
    let mut lock = stdout.lock();
    for result in results {
        writeln!(lock, "{}, {}, {}", result.0, result.1, result.2)?;
    }

    Ok(())
}