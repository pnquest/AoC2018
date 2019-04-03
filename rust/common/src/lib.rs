use std::fs::File;
use std::io::prelude::*;
use std::panic;
use std::str;

pub mod geometry;
pub mod collections;
pub mod summed_area;
pub mod pathfinding;

pub fn file_to_vector<T: str::FromStr>(file_name: &str) -> Result<Vec<T>, String>{
    let mut f = match File::open(file_name) {
        Ok(fl) => fl,
        Err(message) => return Err(format!("There was an error opening the file: {}", message))
    };
    
    let mut contents = String::new();
    match f.read_to_string(&mut contents) {
        Err(message) => return Err(format!("There was an error reading the file: {}", message)),
        _ => {}
    };

    Ok(contents
        .lines()
        .map(|v| {
            match v.parse() {
                Ok(itm) => itm,
                Err(_) => panic!("Could not parse: {}", v)
            }
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uints() {
        let vec:Result<Vec<usize>, String> = file_to_vector("ints.txt");

        assert_eq!(vec.is_ok(), true);

        let vals = vec.unwrap();
        
        let res:usize = vals.into_iter().sum();
        assert_eq!(res, 15);
    }

    #[test]
    fn test_ints() {
        let vec:Result<Vec<isize>, String> = file_to_vector("ints.txt");

        assert_eq!(vec.is_ok(), true);

        let vals = vec.unwrap();
        
        let res:isize = vals.into_iter().sum();
        assert_eq!(res, 15);
    }

    #[test]
    fn test_strings() {
        let vec:Result<Vec<String>, String> = file_to_vector("strings.txt");

        assert_eq!(vec.is_ok(), true);

        let vals = vec.unwrap();

        assert_eq!("ball", vals[0]);
        assert_eq!("apple", vals[1]);
    }

    #[test]
    fn test_floats() {
        let vec:Result<Vec<f64>, String> = file_to_vector("floats.txt");

        assert_eq!(vec.is_ok(), true);

        let result:f64 = vec.unwrap().into_iter().sum();

        assert_eq!(6.66, result);

    }

    #[test]
    fn test_bools() {
        let vec = file_to_vector("bools.txt");

        assert_eq!(vec.is_ok(), true);

        let result:Vec<bool> = vec.unwrap();

        assert_eq!(true, result[0]);
        assert_eq!(false, result[1]);
    }
}