use std::fs::File;
use std::io::prelude::*;


//read file and put it in a useable form
pub fn read_file(path: &str, m: usize) -> Vec<Vec<i32>> {
    let mut dfile: Vec<Vec<i32>> = vec![vec![0; 2]; m];
    let file = File::open(path).expect("Could not open file");
    let buf_reader = std::io::BufReader::new(file).lines();
    //make count for indexing
    let mut count = 0;
    for line in buf_reader {
        let line_str = line.expect("Error reading");
        //split by a simple white space
        let v: Vec<&str> = line_str.trim().split(' ').collect();
        let x = v[0].parse::<i32>().unwrap();
        let y = v[1].parse::<i32>().unwrap();
        dfile[count][0] = x;
        dfile[count][1] = y;
        count += 1;
    }
    return dfile;
}

pub fn data_trim(path: Vec<Vec<i32>>) -> Vec<(usize, usize)> {
    let mut length = 0;
    //no node should be directed at itself, so should stop at first [0,0]
    for i in 0..path.len() {
        let count = i as usize;
        if path[count][0] != path[count][1] {
            length += 1;
        }
        else {
            break
        }
    }
    //create new data vector
    let mut data_better: Vec<(usize,usize)> = vec![(0 as usize,0 as usize);length];
    //all the relevant data should be in the new vector of vectors
    for i in 0..length {    
            data_better[i] = (path[i][0] as usize, path[i][1] as usize);
    }
    //can check with the last data points in the file if it
    //is correct
    return data_better;
}
