use std::collections::HashMap;

pub struct Graph {
    vertices: Vec<usize>, //stores vertex id's
    adjacency_list: HashMap<usize, Vec<usize>>,  
}

impl Graph { 
    //initializes empty graph w/ empty vecors + empty adjacency list
    pub fn new() -> Self {
        Graph {
            vertices: Vec::new(),
            adjacency_list: HashMap::new(),
        }
    }

    //add vertex to graph
    pub fn add_vertex(&mut self, vertex: usize) {
        //checks if vertex already in graph
        if !self.vertices.contains(&vertex) {
            //adds vertex to vertices vector
            self.vertices.push(vertex);
            //inserts empty list for vertex into adjacency list
            self.adjacency_list.insert(vertex, Vec::new());
        }
    }

    //adds edge between to vertices
    pub fn add_edge(&mut self, from: usize, to: usize) {
        //makes sure vertices exist
        self.add_vertex(from);
        self.add_vertex(to);
        //adds vertex to into adjacency list of vertex from
        self.adjacency_list.entry(from).or_insert(Vec::new()).push(to);
    }

    //finds amount of vertices
    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    pub fn get_vertices(&self) -> &Vec<usize> {
        &self.vertices
    }

    //finds neighbors of vertex in graph
    pub fn get_neighbors(&self, vertex: usize) -> Vec<usize> {
        //checks if vertex exists in adj list
        if let Some(neighbors) = self.adjacency_list.get(&vertex) {
            neighbors.clone()  //returns clone of neighbors list
        } else {
            Vec::new()         //returns an empty vec if there are no neighbors
        }
    }
}