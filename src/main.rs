mod parse;
use std::collections::BinaryHeap;
use core::cmp::Reverse;
use std::collections::VecDeque;
use fast_paths;
use fast_paths::InputGraph;
use fast_paths::FastGraph;

type Vertex = usize;
type Edge = Vec<(Vertex, Vertex)>;
type AdjacencyLists = Vec<Vec<Vertex>>;

#[derive(Debug)]
struct Graph {
    n: usize,
    outedges: AdjacencyLists,
}

impl Graph {
    fn add_directed_edges(&mut self, edges:&Edge) {
        for (u,v) in edges {
            self.outedges[*u].push(*v);
        }
    }
    fn sort_graph_lists(&mut self) {
        for l in self.outedges.iter_mut() {
            l.sort();
        }
    }
    fn create_directed(n:usize,mut edges: Edge) -> FastGraph {
        let mut input_graph = InputGraph::new();
        validation(&edges);
        edges.sort();
        for k in edges{
            input_graph.add_edge(k.0,k.1,1); //Fast Path graph struct
        }
        input_graph.freeze();
        let fast_graph = fast_paths::prepare(&input_graph);
        
        return fast_graph;                                     
    }
    
}

fn get_nodes(graph: &FastGraph, s: usize, e: usize) -> Vec<Vertex>{
    
    let shortest_path = fast_paths::calc_path(&graph, s, e);
    let mut weight = 0;
    let mut nodes: Vec<Vertex> = vec![];
    match shortest_path {
        Some(p) => {
            // the weight of the shortest path, always 1
            weight = p.get_weight();
            // all nodes of the shortest path (including source and target)
            nodes = p.get_nodes().to_vec();
        },
        None => {
            // no path has been found (nodes are not connected in this graph)
        }
    }
    return nodes;
}

fn iter_nodes(graph: &FastGraph, n: usize, s: usize) -> Vec<Vec<Vertex>>{
    let mut total: Vec<Vec<Vertex>> = vec![vec![]];
    let nodelist = FastGraph::get_node_ordering(graph);
    for i in 0..s{ //Runs through all pairs of nodes in the sample size
        println!("Current Node: {}, Position: {}!",&nodelist[i],i);
        for j in 0..s{
            let node = get_nodes(&graph, nodelist[i], nodelist[j]);
            if node.is_empty(){}
            else{
                total.push(node.clone());
                
            }
        }
    }
    return total;
}

fn between(v: Vec<Vec<Vertex>>, n: usize) -> Vec<(f64,usize)>{
    let mut b: Vec<(f64,usize)> = vec![(0.0,0);n];
    let mut c: Vec<(usize,usize)> = vec![(0,0);n];
    let len = &v.len();
    for h in 0..n{
        b[h].1 = h;
        c[h].1 = h;

    }
    for i in v{
        for j in i{
            c[j].0 += 1; //totals the amount of times a node shows up in the vector of shortest paths
        }
    }
    c.sort();
    c.reverse();

    for h in 0..10{
        let mut div = c[h].0 as f64 / *len as f64; //computes betweenness centrality: # of time in shortest path/# of shortest paths
        b[h].0 = div;
        println!("Node: {} | Score: {}",c[h].1,div);
    }
    return b;
}

fn validation(edges: &Edge){
    for i in edges{
        let i1 = Some(i.0);
        let i0 = Some(i.1);
        if let None = i0{
            panic!("INVALID EDGES");
        }
        if let None = i1{
            panic!("INVALID EDGES");
        }
    }
}

fn info(v: Vec<(f64,usize)>, nodes: Vec<Vec<Vertex>>, graph: &FastGraph, depth: usize){
    for i in 0..v.len(){
        println!("Node: {} | Centrality: {}", v[i].1,v[i].0);
    }
    println!("{:?}", nodes);
    for j in 0..depth{
        println!("{:?}",graph);
    }
}

fn main() {
    let file_path = "enwiki-2013.txt";
    let sample_size = 100;
    let (size, edges) = parse::read_file(file_path, sample_size);
    println!("Finished Reading!");
    
    let mut graph = Graph::create_directed(size, edges); //Creates InputGr aph struct and uses contraction hierarchies to convert to FastGraph Struct
    println!("Finished Graph Conversion!");
    let n = FastGraph::get_num_nodes(&graph); //Returns the list of nodes that were not pruned by FastGraph
    let nodes = iter_nodes(&graph, n, sample_size);
    let v = between(nodes,n);

    // info(v, nodes, &graph, depth); //Information Dump, not too useful
}
