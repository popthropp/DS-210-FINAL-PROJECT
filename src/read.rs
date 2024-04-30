use std::fs::File;               
use std::io::{BufReader, BufRead}; 

mod graph;

//function for reading a graph from file
pub fn read_graph_from_file(file_path: &Path) -> Result<graph::Graph, String> {
    //opens file
    let file = File::open(file_path).map_err(|e| format!("Failed to open file: {}", e))?;
    let reader = BufReader::new(file);

    //initializes an empty graph
    let mut graph = graph::Graph::new();
    //iterates over each lin in file
    for (index, line) in reader.lines().enumerate() {
        //reads each line and parses vertices
        let line = line.map_err(|e| format!("Failed to read line {}: {}", index + 1, e))?;
        let mut parts = line.split_whitespace();
        let from = parts.next().ok_or_else(|| format!("Missing from vertex in line {}", index + 1))?;
        let to = parts.next().ok_or_else(|| format!("Missing to vertex in line {}", index + 1))?;
        let from: usize = from.parse().map_err(|e| format!("Invalid from vertex in line {}: {}", index + 1, e))?;
        let to: usize = to.parse().map_err(|e| format!("Invalid to vertex in line {}: {}", index + 1, e))?;
        //adds edge to graph
        graph.add_edge(from, to);
    }

    //returns created graph
    Ok(graph)