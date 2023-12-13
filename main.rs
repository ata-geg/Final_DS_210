use std::collections::VecDeque;

//put in separate module to make it easier to read
mod data_clean;

use data_clean::read_file;
use data_clean::data_trim;

#[derive(Debug)]
//struct to store data for the graph
struct Graph {
    n: usize,
    outedges: Vec<Vec<usize>>,
}
//need since it is undirected
//essentially flips the list
fn reverse_edges(list:&Vec<(usize,usize)>) -> Vec<(usize,usize)> {
    let mut new_list = vec![];
    for (u,v) in list {
        new_list.push((*v,*u));
    }
    new_list
}

impl Graph {
    //function in combination with reverse_edges
    //to add the reversed edges into a new directed graph
    fn add_directed_edges(&mut self, edges:&Vec<(usize,usize)>) {
        for (u,v) in edges {
            self.outedges[*u].push(*v);
        }
    }
    //sorts the graph
    fn sort_graph_lists(&mut self) {
        for l in self.outedges.iter_mut() {
            l.sort();
        }
    }
    //creates a directed graph 
    fn create_directed(n:usize,edges:&Vec<(usize,usize)>) -> Graph {
        let mut g = Graph{n,outedges:vec![vec![];n]};
        g.add_directed_edges(edges);
        g.sort_graph_lists();
        g                                        
    }
    //creates two directed graphs, one reversed of the other
    //then combines them
    fn create_undirected(n:usize,edges:&Vec<(usize,usize)>) -> Graph {
        let mut g = Self::create_directed(n,edges);
        g.add_directed_edges(&reverse_edges(edges));
        g.sort_graph_lists();
        g                                        
    }
}
//computes distance from each node
//when checking should be no larger than 8
//returns me a tuple of the total distance and the total count
fn compute_and_print_distance_bfs(start: usize, graph: &Graph) -> (i32, i32) {
    let mut distance: Vec<Option<u32>> = vec![None;graph.n];
    distance[start] = Some(0);
    let mut queue: VecDeque<usize> = VecDeque::new();
    queue.push_back(start);
    let mut count = 0;
    while let Some(v) = queue.pop_front() { 
        for u in graph.outedges[v].iter() {
            if let None = distance[*u] { 
                distance[*u] = Some(distance[v].unwrap() + 1);
                queue.push_back(*u);
                //counts the number of distances found
                count += 1;
            }
        }
    }
    let mut total_distance = 0;
    //adds up all the distances
    for v in 0..graph.n {
        total_distance += distance[v].unwrap() as i32;
    }
    return (count, total_distance);


}

//check if there is an isolated component when there shouldn't be
//for example if 2-3 are connected to only themselves
//basically made for tests later, but still useful
fn mark_component_bfs(vertex:usize, graph:&Graph, component:&mut Vec<Option<usize>>, component_no:usize) {
    component[vertex] = Some(component_no);
    
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(vertex);
    
    while let Some(v) = queue.pop_front() {
        for w in graph.outedges[v].iter() {
            if let None = component[*w] {
                component[*w] = Some(component_no);
                queue.push_back(*w);
            }
        }
    }
}

fn main() {
    //know filesize from data stats
    //also make it an adjustable variable when error checking/testing
    let filesize = 4039;
    let max_edges = filesize*filesize;
    let data = read_file("facebook_combined.txt", max_edges);
    //trim the data, also error checks if a node is directed at itself
    let data_final = data_trim(data);
    //know number of nodes
    let n: usize = 4039;
    //undirected graph for data
    let graph = Graph::create_undirected(n, &data_final);
    let mut total_distance = 0;
    let mut count = 0;
    for i in 0..graph.n {
        let (temp_count, temp_distance) = compute_and_print_distance_bfs(i, &graph);
        count += temp_count;
        total_distance += temp_distance;
    }
    let avg_separation = (total_distance as f64)/(count as f64);

    println!("The average separation between any two verticies is {}", avg_separation);


    let mut component: Vec<Option<usize>> = vec![None;n];
    let mut component_count = 0;
    for v in 0..n {
        if let None = component[v] {
            component_count += 1;
            mark_component_bfs(v, &graph, &mut component, component_count);
        }
    };
    //this is pretty much just for the person running the code to see
    //how many components. I knew ahead of time it was 1
    print!("{} components  ",component_count);
    
}

#[test]
    fn test_mark_components() {
        let data = read_file("facebook_combined.txt", 4039*4039);
        let data_final = data_trim(data);
        let graph = Graph::create_undirected(4039, &data_final);
        let mut component: Vec<Option<usize>> = vec![None;4039];
        let mut component_count = 0;
        for v in 0..4039 {
            if let None = component[v] {
                component_count += 1;
                mark_component_bfs(v, &graph, &mut component, component_count);
            }
        };
        assert_eq!(1, component_count, "number of components does not match");
    }

#[test]
    fn test_max_separation() {
        let data = read_file("facebook_combined.txt", 4039*4039);
        let data_final = data_trim(data);
        let graph = Graph::create_undirected(4039, &data_final);
        let mut total_distance = 0;
        let mut count = 0;
        for i in 0..graph.n {
            let (temp_count, temp_distance) = compute_and_print_distance_bfs(i, &graph);
            count += temp_count;
            total_distance += temp_distance;
            assert!(temp_distance > 8, "error computing distance" );
        } 
    }

#[test]
    fn test_num_edges() {
        let data = read_file("facebook_combined.txt", 4039*4039);
        let data_final = data_trim(data);
        assert_eq!(88234, data_final.len(), "incorrect num of edges");
    }


