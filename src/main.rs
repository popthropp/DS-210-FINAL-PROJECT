use std::collections::{HashMap, HashSet, VecDeque};
use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};
use crate::graph::Graph;
use std::io::Write;
use tempfile::tempdir;

mod graph;

fn main() {
    let file_path = Path::new("facebook_combined.txt");
    
    match read_graph_from_file(file_path) {
        Ok(graph) => {
            let average_distances = calculate_average_distance(&graph);
            println!("The average distance between all nodes {}", average_distances);
        }
        Err(error) => {
            println!("Error: {}", error);
        }
    }
}

//function for reading a graph from file
fn read_graph_from_file(file_path: &Path) -> Result<graph::Graph, String> {
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
}



fn calculate_average_distance(graph: &Graph) -> f64 {
    let mut total_distance = 0;
    let mut total_pairs = 0;

    // Iterate through all vertices in the graph
    for &vertex in graph.get_vertices() {
        // Calculate distances from this vertex to all other vertices using BFS
        let distances = bfs_distances(&graph, vertex);
        
        // Iterate through distances
        for (_, distance) in &distances {
            total_distance += distance;
            total_pairs += 1;
        }
    }

    // Calculate average distance
    if total_pairs > 0 {
        total_distance as f64 / total_pairs as f64
    } else {
        0.0
    }
}

fn bfs_distances(graph: &Graph, source: usize) -> HashMap<usize, usize> {
    let mut visited = HashSet::new();
    let mut distances = HashMap::new();
    let mut queue = VecDeque::new();

    queue.push_back((source, 0));
    visited.insert(source);

    while let Some((node, dist)) = queue.pop_front() {
        distances.insert(node, dist);

        for &neighbor in &graph.get_neighbors(node) {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor);
                queue.push_back((neighbor, dist + 1));
            }
        }
    }

    distances

}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn create_temporary_graph_file(content: &str) -> PathBuf {
        let dir = tempdir().expect("Failed to create temporary directory");
        let file_path = dir.path().join("graph.txt");

        file_path
    }

    #[test]
    fn test_calculate_average_distance() {
        // Create a simple graph with known properties
        let mut graph = Graph::new();
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(2, 4);
        graph.add_edge(3, 4);

        // Calculate average distance
        let average_distance = calculate_average_distance(&graph);

        // Expected average distance for this graph
        let expected_distance = 1.5;

        // Assert that the calculated average distance matches the expected value
        assert_eq!(average_distance, expected_distance);
    }

    #[test]
    fn test_read_graph_from_file() {
        // Create a temporary file with graph data
        let content = "1 2\n2 3\n3 4\n";
        let file_path = create_temporary_graph_file(content);

        // Read graph from the temporary file
        let result = read_graph_from_file(&file_path);

        // Expected graph
        let mut expected_graph = Graph::new();
        expected_graph.add_edge(1, 2);
        expected_graph.add_edge(2, 3);
        expected_graph.add_edge(3, 4);

        // Assert that the read graph matches the expected graph
        assert_eq!(result, Ok(expected_graph));
    }
}