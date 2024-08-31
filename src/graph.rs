/// Graph
use std::collections::HashMap;

struct Graph {
    edges: HashMap<String, Vec<String>>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            edges: HashMap::new(),
        }
    }

    fn add_vertex(&mut self, vertex: &str) {
        self.edges.entry(vertex.to_string()).or_insert(Vec::new());
    }

    fn add_edge(&mut self, from: &str, to: &str) {
        self.edges
            .entry(from.to_string())
            .or_insert(Vec::new())
            .push(to.to_string());
    }

    fn adjacents(&self, vertex: &str) -> Option<&Vec<String>> {
        self.edges.get(vertex)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_vertex() {
        let mut graph = Graph::new();

        let vertex = "A";
        graph.add_vertex(vertex);

        assert_eq!(graph.adjacents(vertex).unwrap().len(), 0);
    }

    #[test]
    fn add_edge() {
        let mut graph = Graph::new();

        let vertex_a = "A";
        graph.add_vertex(vertex_a);

        let vertex_b = "B";
        graph.add_vertex(vertex_b);

        graph.add_edge(vertex_a, vertex_b);

        assert_eq!(graph.adjacents(vertex_a).unwrap().len(), 1);
        assert_eq!(graph.adjacents(vertex_a).unwrap()[0], vertex_b);
    }
}
