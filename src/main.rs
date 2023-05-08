mod read_file;
mod graph;
mod bfs;

type Vertex = usize;
type ListOfEdges = Vec<(Vertex,Vertex)>;
type AdjacencyLists = Vec<Vec<Vertex>>;

fn count_degree(result: Vec<i32>) 
                     -> Vec<[usize; 2]>{
    let mut count = vec![[0,result.len()],[1,0],[2,0],[3,0],[4,0],[5,0],[6,0],[7,0]];
    for i in 0..result.len() {
        if result[i] == 0 {
            count[0][1] -= 1
        } else if result[i] == 1 {
            count[1][1] += 1
        } else if result[i] == 2 {
            count[2][1] += 1
        } else if result[i] == 3 {
            count[3][1] += 1
        } else if result[i] == 4 {
            count[4][1] += 1
        } else if result[i] == 5 {
            count[5][1] += 1        
        } else if result[i] == 6 {
            count[6][1] += 1
        } else if result[i] > 6 {
            count[7][1] += 1
            
        }    // aims to count the number of different degrees of separations
    }        // 0 minus points with 0 connections
                         
    return count;
}


fn main() {
    // a short main function to combine all modules to get a result
    let mut read_edges = read_file::read_file("facebook_combined.txt");
    let mut graph = graph::Graph::create_directed(4039, &read_edges);
    let mut result: Vec<i32> = vec![];
    let total_dis: usize = 88234;
    let total_average_dis: f64 = total_dis as f64/4039 as f64;
    let mut count_node = vec![];
    let mut last_node = vec![];
    
    for i in 0..4039 {
        result.push(bfs::bfs_computation(i, &graph));   
    }   
    println!("Total of 4039 nodes, 88234 edges: ");
    println!("The average distance from nodes to all other nodes connected is: ");
    println!("{:?}",result);
    println!("From statistics aspect: {:?}", count_degree(result));

    for (i, l) in graph.outedges.iter().enumerate(){
        count_node.push(l.len());
        last_node.push(i);
    }
    count_node.sort();
    last_node.sort();
    
    let min_value: usize = *count_node.iter().min().unwrap();
    let max_value: usize = *count_node.iter().max().unwrap();
    
    println!("Last Node: {}", last_node[last_node.len()-1]);
    println!("Min Connections: {}", min_value);
    println!("Max Connections: {}", max_value);
    println!("Total Average Distance(Wrong, just for reference): {}", total_average_dis); 
}


#[test]
fn test_directed_edges() {
    let mut edges: ListOfEdges = vec![(0,2),(1,2),(2,4),(2,6)];
    let mut graph = graph::Graph::create_directed(3,&edges);
    let mut count_edge = vec![];
    
    for (_i, j) in graph.outedges.iter().enumerate(){
        count_edge.push(j.len());
    }
    assert_eq!(count_edge,[1,1,2]);
}

#[test]
fn test_count_degree_sum(){
    let count_sample = vec![1,1,2,2,2,4,8];
    let count_test = count_degree(count_sample);
    assert_eq!(count_test, vec![[0,7],[1,2],[2,3],[3,0],[4,1],[5,0],[6,0],[7,1]]);
}