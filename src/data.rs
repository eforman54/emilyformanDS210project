use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn read(file_path: &Path, max_nodes: Option<usize>) -> HashMap<usize, HashSet<usize>> {
   //open file, return error if cannot be opened
   let file = File::open(&file_path).expect("Cannot open file");
   let reader = BufReader::new(file);

   //empty hashmap
   let mut graph: HashMap<usize, HashSet<usize>> = HashMap::new();
   for line in reader.lines() {
       let line = line.expect("Cannot read line");
       if !line.starts_with("#") {         
           //nodes into a vector
           let nodes: Vec<&str> = line.split_whitespace().collect();
           let from = nodes[0].parse::<usize>().unwrap();
           let to = nodes[1].parse::<usize>().unwrap();
           
           if let Some(max) = max_nodes {
               if from >= max || to >= max {
                   continue;
               }
           }
           graph.entry(from).or_insert(HashSet::new()).insert(to);
          
       }
   }
   graph
}
