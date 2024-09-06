/// K-Nearest Neighbors
///
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Debug)]
struct Point {
    coords: Vec<f64>,
    label: String,
}

fn euclidean_distance(p1: &Point, p2: &Point) -> f64 {
    p1.coords
        .iter()
        .zip(&p2.coords)
        .map(|(a, b)| (a - b).powi(2))
        .sum::<f64>()
        .sqrt()
}

#[derive(Debug)]
struct Neighbor {
    distance: f64,
    label: String,
}

impl Eq for Neighbor {}

impl PartialEq for Neighbor {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl Ord for Neighbor {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.partial_cmp(&self.distance).unwrap()
    }
}

impl PartialOrd for Neighbor {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn knn(training_set: &[Point], test_point: &Point, k: usize) -> String {
    let mut heap = BinaryHeap::new();

    for point in training_set {
        let distance = euclidean_distance(point, test_point);
        heap.push(Neighbor {
            distance,
            label: point.label.clone(),
        });

        if heap.len() > k {
            heap.pop();
        }
    }

    let mut label_count = HashMap::new();
    for neighbor in heap {
        *label_count.entry(neighbor.label).or_insert(0) += 1;
    }

    label_count
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .unwrap()
        .0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn euclidean_distance_test() {
        let p1 = Point {
            coords: vec![1.0, 2.0],
            label: "p1".to_string(),
        };
        let p2 = Point {
            coords: vec![3.0, 4.0],
            label: "p2".to_string(),
        };

        let distance = euclidean_distance(&p1, &p2);
        let result: f64 = format!("{:.1}", distance).parse().unwrap();
        assert_eq!(result, 2.8);
    }

    #[test]
    fn knn_test() {
        let training_set = vec![
            Point {
                coords: vec![1.0, 2.0],
                label: "A".to_string(),
            },
            Point {
                coords: vec![2.0, 3.0],
                label: "A".to_string(),
            },
            Point {
                coords: vec![3.0, 3.0],
                label: "B".to_string(),
            },
            Point {
                coords: vec![5.0, 4.0],
                label: "B".to_string(),
            },
        ];

        let test_point = Point {
            coords: vec![2.5, 2.5],
            label: "".to_string(),
        };

        let result = knn(&training_set, &test_point, 3);
        assert_eq!(result, String::from("B"));
    }
}
