use std::fs;
use std::time::Instant;

const POKEDEX_LENGTH: usize = 70; // MAX 898

fn main() {
    let file = fs::read_to_string("./pokedex.json").expect("Unable to read file");
    let pokedex: Vec<String> = serde_json::from_str(&file).expect("JSON was not properly formatted.");  
    let graph = create_graph(&pokedex[0..POKEDEX_LENGTH].to_vec());

    let start = Instant::now();
    let pokechain = find_longest_pokechain(&graph, pokedex);
    let duration = start.elapsed().as_millis();

    println!("Pokechain: {:?}", pokechain);
    println!("Length of chain: {:?}", pokechain.len());
    println!("Duration: {:?} ms", duration);

}


fn create_graph(pokedex: &Vec<String>) -> [[u32; POKEDEX_LENGTH]; POKEDEX_LENGTH] {
    let mut graph: [[u32; POKEDEX_LENGTH]; POKEDEX_LENGTH] = [[0; POKEDEX_LENGTH]; POKEDEX_LENGTH];

    for i in 0..pokedex.len() {
        for j in 0..pokedex.len() {
            if pokedex[i].chars().last() == pokedex[j].chars().next() && i != j {
                graph[i][j] = 1;
            }
        }
    }
    return graph;
}

fn dft(node: usize, graph: &[[u32; POKEDEX_LENGTH]; POKEDEX_LENGTH], visited: &[u32; POKEDEX_LENGTH]) -> Vec<usize> {
    let mut longest_neighbor: Vec<usize> = Vec::new();
    let mut new_visited = visited.clone();
    new_visited[node] = 1;

    for i in 0..graph[node].len() {
        if graph[node][i] == 1 && visited[i] == 0 {
            let neighbor_chain = dft(i, &graph, &new_visited);
            if neighbor_chain.len() > longest_neighbor.len() {
                longest_neighbor = neighbor_chain.clone();
            }
        }
    }

    let mut longest_chain = vec![node];
    longest_chain.append(&mut longest_neighbor);

    return longest_chain;
}

fn find_longest_pokechain(graph: &[[u32; POKEDEX_LENGTH]; POKEDEX_LENGTH], pokedex: Vec<String>) -> Vec<String> {
    let mut longest_index_chain: Vec<usize> = Vec::new();
    for i in 0..graph.len() {
        let current_index_chain = dft(i, &graph, &[0; POKEDEX_LENGTH]);

        if current_index_chain.len() > longest_index_chain.len() {
            longest_index_chain = current_index_chain.clone();
        }
    }
    
    return longest_index_chain.to_vec().iter().map(|x| { pokedex[*x].clone() }).collect();
}
