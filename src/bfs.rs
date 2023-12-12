
use std::collections::{HashMap, HashSet, VecDeque};
use rand::seq::SliceRandom;
//fidn shortest path between nodes 
pub fn shortest_path(
    graph: &HashMap<usize, HashSet<usize>>,
    start: usize,
    end: usize,
) -> Option<usize> { //dequeues current node from front of queue
    let mut already_visited = HashMap::new();
    let mut queue = VecDeque::new();
    already_visited.insert(start, 0);
    queue.push_back(start);


    while let Some(current) = queue.pop_front() {
        if current == end { //when end node is reached, return shortest path distance 
            return already_visited.get(&current).cloned();
        }

//next, visit node neighbors
        if let Some(neighbors) = graph.get(&current) {
            for neighbor in neighbors {
                //if neighbor not yet visited, add to queue and set the distance 
                if !already_visited.contains_key(neighbor) {
                    queue.push_back(*neighbor);
                    already_visited.insert(*neighbor, already_visited[&current] + 1);
                }
            }
        }
    }
    None
}

//average shortest path length 
pub fn shortest_path_length(graph: &HashMap<usize, HashSet<usize>>) -> f64 {
    let mut totallen = 0;
    let mut numberpaths = 0;
    let mut iteration_count = 0;

    for &start in graph.keys() {
        for &end in graph.keys() {
            if start != end {
                if let Some(length) = shortest_path(graph, start, end) {
                    totallen += length;
                    numberpaths += 1;
                }
            }
        iteration_count = iteration_count + 1;
        }
    }
//return average 
    (totallen as f64) / (numberpaths as f64)
}

//find the top 5 nodes with incoming edges 
pub fn top5nodes(graph: &HashMap<usize, HashSet<usize>>) {
    let mut incoming_count: HashMap<usize, usize> = HashMap::new();
    //iterate through each node and edges 
    for (_node, edges) in graph.iter() {
        for edge in edges {
            *incoming_count.entry(*edge).or_insert(0) += 1;
        }
    }
//sort by decreasing order 
    let mut nodes_edge: Vec<(usize, usize)> = incoming_count.into_iter().collect();
    nodes_edge.sort_unstable_by(|a, b| b.1.cmp(&a.1)); 
    println!("The top 5 nodes with the highest number of incoming edges are:");
    for (i, (node, count)) in nodes_edge.iter().take(5).enumerate() {
        println!("{}. Node {}: with {} incoming edges", i + 1, node, count);
    }
}

// this function will calculate the percentage of nodes with incoming edges in a given range
pub fn percent_nodes(graph: &HashMap<usize, HashSet<usize>>, min_edges: usize, max_edges: usize) -> f64 {
    let mut nodes_in_range = 0;
    for &node in graph.keys() {
        if let Some(edges) = graph.get(&node) {
            let incoming_edges_count = edges.len();
            if incoming_edges_count >= min_edges && incoming_edges_count <= max_edges {
                nodes_in_range += 1;
            }
        }
    }
    let total_nodes = graph.len() as f64;
    let percentage = (nodes_in_range as f64 / total_nodes) * 100.0;
    percentage
}



