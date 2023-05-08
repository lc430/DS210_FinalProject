use std::collections::VecDeque;
use crate::graph::Graph;

type Vertex = usize;

pub fn bfs_computation(start: Vertex, graph: &Graph) -> i32{
    let mut distance: Vec<Option<u32>> = vec![None;graph.n];
    distance[start] = Some(0); 
    let mut queue: VecDeque<Vertex> = VecDeque::new();
    queue.push_back(start);
    
    let mut distance_cleaned: Vec<u32> = vec![];
    let mut counter: u32 = 0;
    let mut average_distance: i32 = 0;

    while let Some(v) = queue.pop_front() { 
        for u in graph.outedges[v].iter() {
            if let None = distance[*u] { 
                distance[*u] = Some(distance[v].unwrap() + 1);
                queue.push_back(*u);
            }
        }
    }
        
    for v in 0..graph.n {
        if distance[v] != None {
            distance_cleaned.push(distance[v].unwrap());
        }
    }
    for i in &distance_cleaned {
        counter += *i;
        average_distance = (counter/(distance_cleaned.len() as u32)) as i32;
    }
    return average_distance;
}