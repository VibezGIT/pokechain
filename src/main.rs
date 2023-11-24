use std::fs;
use std::collections::HashMap;
use std::time::Instant;

struct Node {
    pokemon: String,
    edges: Vec<usize>
}


fn main() {
    let pokedex_length = 70; //MAX 898

    let file = fs::read_to_string("./pokedex.json").expect("Unable to read file");
    let pokedex: Vec<String> = serde_json::from_str(&file).expect("JSON was not properly formatted.");  
    let graph = create_graph(&pokedex[0..pokedex_length].to_vec());

    let start = Instant::now();
    let pokechain = find_longest_pokechain(&graph);
    let duration = start.elapsed().as_millis();

    println!("Pokechain: {:?}", pokechain);
    println!("Length of chain: {:?}", pokechain.len());
    println!("Duration: {:?}s", duration);

}


fn create_graph(pokedex: &Vec<String>) -> Vec<Node> {
    let mut graph: Vec<Node> = Vec::new();
    let pokedex =  pokedex.clone();

    for i in 0..pokedex.len() {
        let pokemon = pokedex[i].clone();
        let mut edges: Vec<usize> = Vec::new();

        for j in 0..pokedex.len() {
            if pokedex[i].chars().last() == pokedex[j].chars().next() {
                edges.push(j);
            }
        }

        graph.push(Node{pokemon, edges});
    }

    return graph;
}

fn dft(node: &Node, graph: &Vec<Node>, visited: &HashMap<usize, bool>) -> Vec<String> {
    let mut longest_neighbor: Vec<String> = Vec::new();
    let edges_copy = node.edges.clone();

    for i in 0..edges_copy.len() {
        let mut new_visited = visited.clone();
        match new_visited.insert(edges_copy[i], true) {
            None => {
                let neighbor_chain = dft(&graph[edges_copy[i]], graph, &new_visited);
        
                if neighbor_chain.len() > longest_neighbor.len() {
                    longest_neighbor = neighbor_chain.clone();
                }
            }
            Some(_) => { continue }
        }
    }

    let mut longest_chain = vec![node.pokemon.clone()];
    longest_chain.append(&mut longest_neighbor);

    return longest_chain;
}

fn find_longest_pokechain(graph: &Vec<Node>) -> Vec<String> {
    let mut longest_chain: Vec<String> = Vec::new();
    for i in 0..graph.len() {
        let mut visited = HashMap::new();
        visited.insert(i, true);
        let current_chain = dft(&graph[i], graph, &visited);

        if current_chain.len() > longest_chain.len() {
            longest_chain = current_chain.clone();
        }
    }

    return longest_chain;
}

