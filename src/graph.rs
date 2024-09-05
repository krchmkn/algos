/// Graph
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::usize::MAX;

struct Graph {
    edges: HashMap<String, Vec<(String, usize)>>,
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

    fn add_edge(&mut self, from: &str, to: &str, weight: usize) {
        self.edges
            .entry(from.to_string())
            .or_insert(Vec::new())
            .push((to.to_string(), weight));
    }

    fn adjacents(&self, vertex: &str) -> Option<&Vec<(String, usize)>> {
        self.edges.get(vertex)
    }

    fn bfs<'a>(&'a self, start: &'a str) -> HashSet<&'a str> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        visited.insert(start);
        queue.push_back(start);

        while let Some(node) = queue.pop_front() {
            if let Some(neighbors) = self.edges.get(node) {
                for neighbor in neighbors {
                    if visited.insert(&neighbor.0) {
                        queue.push_back(&neighbor.0);
                    }
                }
            }
        }

        visited
    }

    fn dfs<'a>(&'a self, node: &'a str, visited: &mut HashSet<&'a str>) {
        if !visited.insert(node) {
            return;
        }

        if let Some(neighbors) = self.edges.get(node) {
            for neighbor in neighbors {
                self.dfs(&neighbor.0, visited);
            }
        }
    }

    fn dijkstra(&self, start: &str) -> HashMap<String, usize> {
        let mut distances: HashMap<&str, usize> =
            self.edges.keys().map(|v| (v.as_str(), MAX)).collect();
        let mut heap = BinaryHeap::new();

        distances.insert(start, 0);
        heap.push(State {
            vertex: start,
            cost: 0,
        });

        while let Some(State { vertex, cost }) = heap.pop() {
            if cost > distances[vertex] {
                continue;
            }

            if let Some(neighbors) = self.adjacents(vertex) {
                for neighbor in neighbors {
                    let neighbor_str = neighbor.0.as_str();
                    let next_cost = cost + neighbor.1;

                    if next_cost < *distances.get(neighbor_str).unwrap_or(&MAX) {
                        distances.insert(neighbor_str, next_cost);
                        heap.push(State {
                            vertex: neighbor_str,
                            cost: next_cost,
                        });
                    }
                }
            }
        }

        distances
            .iter()
            .map(|(&k, &v)| (k.to_string(), v))
            .collect()
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State<'a> {
    vertex: &'a str,
    cost: usize,
}

impl<'a> Ord for State<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl<'a> PartialOrd for State<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
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

        graph.add_edge(vertex_a, vertex_b, 1);

        assert_eq!(graph.adjacents(vertex_a).unwrap().len(), 1);
        assert_eq!(graph.adjacents(vertex_a).unwrap()[0].0, vertex_b);
    }

    #[test]
    fn bfs() {
        let mut graph = Graph::new();

        graph.add_vertex("A");
        graph.add_vertex("B");
        graph.add_vertex("C");
        graph.add_vertex("D");

        graph.add_edge("A", "B", 1);
        graph.add_edge("B", "C", 2);
        graph.add_edge("C", "D", 3);
        graph.add_edge("D", "A", 4);

        let visited = graph.bfs("A");
        assert_eq!(visited.len(), 4);
        assert!(visited.contains("A"));
        assert!(visited.contains("B"));
        assert!(visited.contains("C"));
        assert!(visited.contains("D"));
    }

    #[test]
    fn dfs() {
        let mut graph = Graph::new();

        graph.add_vertex("A");
        graph.add_vertex("B");
        graph.add_vertex("C");
        graph.add_vertex("D");

        graph.add_edge("A", "B", 1);
        graph.add_edge("B", "C", 2);
        graph.add_edge("C", "D", 3);
        graph.add_edge("D", "A", 4);

        let mut visited = HashSet::new();

        graph.dfs("A", &mut visited);

        println!("{:?}", &visited);
        assert_eq!(visited.len(), 4);
        assert!(visited.contains("A"));
        assert!(visited.contains("B"));
        assert!(visited.contains("C"));
        assert!(visited.contains("D"));
    }

    #[test]
    fn dijkstra() {
        let mut graph = Graph::new();

        graph.add_vertex("A");
        graph.add_vertex("B");
        graph.add_vertex("C");
        graph.add_vertex("D");
        graph.add_vertex("E");

        graph.add_edge("A", "B", 1);
        graph.add_edge("A", "C", 4);
        graph.add_edge("B", "C", 2);
        graph.add_edge("B", "D", 5);
        graph.add_edge("C", "D", 1);
        graph.add_edge("D", "E", 4);

        let distances = graph.dijkstra("A");

        println!("{:?}", distances);

        assert_eq!(distances["A"], 0);
        assert_eq!(distances["B"], 1);
        assert_eq!(distances["C"], 3);
        assert_eq!(distances["D"], 4);
        assert_eq!(distances["E"], 8);
    }
}
