use std::collections::{HashMap, HashSet, VecDeque};
use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};
use crate::graph::Graph;

mod graph;

fn main() {
    //Defines file path
    let file_path = Path::new("facebook_combined.txt"); 
    
    //Reads graph from file
    match read_graph_from_file(file_path) {

        //If read is successful then calculates average distances of graph
        Ok(graph) => {
            let average_distances = calculate_average_distance(&graph);

            //Prints final output of average distances from every node
            println!("The average distance between all nodes is {:.3}", average_distances);
        }
        //If there's an error during the process it'll print an error message
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

    //Iterate through every vertice
    for &vertex in graph.get_vertices() {
        //Finds distance from current vertex to every vertex using BFS
        let distances = bfs_distances(&graph, vertex);
        
        //Iterates through every distance
        for (_, distance) in &distances {
            total_distance += distance;
            total_pairs += 1;
        }
    }

    //Calculates average distance
        total_distance as f64 / total_pairs as f64
    }

fn bfs_distances(graph: &Graph, source: usize) -> HashMap<usize, usize> {

    //Initializes Hashset, Hashmap, and Queue
    let mut visited = HashSet::new();
    let mut distances = HashMap::new();
    let mut queue = VecDeque::new();

    //Initializes source node and marks it visited
    queue.push_back((source, 0));
    visited.insert(source);

    //Will run until queue is empty
    while let Some((node, dist)) = queue.pop_front() {

        //Inserts distance from source node to current node
        distances.insert(node, dist);

        //Iterates over neigbors of current node and checks if the neighbor has or hasn't been visited
        //If it has, it goes back to the beginning of the loop but if not then it marks the neighbor as visited and pushes it into the queue
        for &neighbor in &graph.get_neighbors(node) {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor);
                queue.push_back((neighbor, dist + 1));
            }
        }
    }
    
    //returns Hashmap of {Node,distance}
    distances
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_calculate_average_distance() {
        //Create a sample graph with known properties
        let mut graph = Graph::new();
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(2, 4);
        graph.add_edge(3, 4);

        let average_distance = calculate_average_distance(&graph);

        //Expected average distance for this graph
        let expected_distance = 0.667;

        //Check if expected and calculated average distances are equal (Rounded calculated distance to 3 decimal places)
        assert_eq!((average_distance * 1000.0).round()/1000.0, expected_distance);
    }

    #[test]
    fn test_bfs_distances() {
        //Create a sample graph
        let mut graph = Graph::new();
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(2, 4);
        graph.add_edge(3, 4);

        //Call bfs_distances with vertex 1 as the source
        let distances = bfs_distances(&graph, 1);

        //Expected distances from vertex 1 to other vertices
        let expected_distances: HashMap<usize, usize> = [(1, 0), (2, 1), (3, 1), (4, 2)]
            .iter()
            .cloned()
            .collect();

        //Check if expected and calculated distance are equal
        assert_eq!(distances, expected_distances);
        }
    }