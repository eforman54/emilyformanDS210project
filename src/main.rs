#![allow(unused_imports)]
use std::io::Write;
use std::fs::File;
use tempfile::tempdir;
use std::collections::{HashMap, HashSet};

mod bfs;
mod data;
use bfs::{shortest_path_length, top5nodes, percent_nodes};
use data::read;
use std::path::Path;

fn main() {
    let file_path = Path::new("email-EuAll.txt");
    //setting a limit for the max nodes 
    let max_nodes = Some(1000);
    let graph = read(&file_path, max_nodes);
    let averagelen = shortest_path_length(&graph);
    //show the top 5 nodes in the graph and average shortest path length
    top5nodes(&graph);
    // the ranges of edges i want to check compared to the percentage of the nodes that have that corresponding range
    let edge_ranges = [(0, 10), (11, 20), (21, 30), (31, 40), (41, 50), (51, 60), (61,70), (71,80), (81,90), (91,100), (101,110), (111, 120)];
    for &(min_edges, max_edges) in &edge_ranges {
        let percentage = percent_nodes(&graph, min_edges, max_edges);
        println!("The percentage of nodes with a range of {} to {} edges: {:.2}%", min_edges, max_edges, percentage);
    }
    println!("Average shortest path length: {:.2}", averagelen);
    
}
//testing the read function 
#[test]
fn test_read() {
    let dir = tempdir().expect("Failed to create temp dir");
    let file_path = dir.path().join("test_data.txt");
    let data = "1 2\n2 3\n3 1";
    let mut file = File::create(&file_path).expect("Failed to create test data file");
    file.write_all(data.as_bytes()).expect("Failed to write test data");
    let result = read(file_path.as_path(), None);

    // check if the graph is correct
    let mut expected: HashMap<usize, HashSet<usize>> = HashMap::new();
    expected.insert(3, [1].iter().cloned().collect());
    expected.insert(2, [3].iter().cloned().collect());
    expected.insert(1, [2].iter().cloned().collect());

    assert_eq!(result, expected);
}
//test for node count since we know there should be 265214 nodes 
#[test]
fn test_node_count() {
    //set the path of the input data
    let file_path = Path::new("email-EuAll.txt");
    //set the maximum number of nodes to read 
    let maximum_nodes = Some(265214);
    //read the graph from the input data
    let graph = read(&file_path, maximum_nodes);
//check if the number of nodes in the graph is correct 
    let expectednode_count = 265214;
    let actualnode_count: usize = graph.len();
    assert!(
        expectednode_count >= actualnode_count,
        "Node count in the graph is incorrect."
    );
}
// testing that the total sum of percentages is 100
#[test]
    fn test_percent_nodes() -> Result<(), Box<dyn std::error::Error>> {
        let mut graph: HashMap<usize, HashSet<usize>> = HashMap::new();
        graph.insert(1, [2, 3].iter().cloned().collect());
        graph.insert(2, [1, 3].iter().cloned().collect());
        graph.insert(3, [1, 2].iter().cloned().collect());
        //all the ranges I chose 
        let edge_ranges = [(0, 10), (11, 20), (21, 30), (31, 40), (41, 50), (51, 60), (61,70), (71,80), (81,90), (91,100), (101,110), (111, 120)];
        let mut total_percentage = 0.0;
        for &(min_edges, max_edges) in &edge_ranges {
            let percentage = percent_nodes(&graph, min_edges, max_edges);
            total_percentage += percentage;
            assert!(percentage >= 0.0 && percentage <= 100.0);
        }
        assert!((total_percentage as f64 - 100.0).abs() < f64::EPSILON);

        Ok(())
    }