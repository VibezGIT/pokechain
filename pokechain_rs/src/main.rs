use std::fs;
use std::fmt;

struct Node {
    pokemon: String,
    edges: Vec<usize>
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Node")
            .field("pokemon", &self.pokemon)
            .field("edges", &self.edges.len())
            .finish()
    }
}

fn main() {
    let file = fs::read_to_string("../pokedex.json").expect("Unable to read file");
    let json: serde_json::Value = serde_json::from_str(&file).expect("JSON was not properly formatted.");
    let length = json.as_array().unwrap().len();
    let mut pokechain: Vec<String> = Vec::new();

    for i in 0..length {
        let current_chain = start(i);
        println!("{} - {}/{}", current_chain.len(), i+1, length);
        if pokechain.is_empty() || current_chain.len() > pokechain.len(){
            pokechain = current_chain;
        }
    }

    let result = serde_json::to_string(&pokechain);
    println!("Found chain of {}!\nPrinted to pokechain.json", pokechain.len());
    fs::write("./pokechain.json", result.unwrap()).expect("Unable to write file.");
}
fn start(index: usize) -> Vec<String> {
    let file = fs::read_to_string("../pokedex.json").expect("Unable to read file");
    let json: serde_json::Value = serde_json::from_str(&file).expect("JSON was not properly formatted.");
    let pokedex: Vec<String> = json.clone().to_owned().as_array_mut().unwrap().iter()
        .map(|x|x.as_str().unwrap().to_owned()).collect();
    let mut graph: Vec<Node> = json.to_owned().as_array_mut().unwrap().iter()
        .map(|x| {
            Node {
                pokemon : x.as_str().unwrap().to_owned(),
                edges : Vec::new()
            }
        }).collect();
    let length = graph.len();

    for i in 0 .. length {
        for j in 0 .. length {
            let poke_a = &pokedex[i];
            let poke_b = &pokedex[j];
            if i != j && poke_a.chars().last() == poke_b.chars().next() {
                graph[i].edges.push(j)
            }
        }
    }

    let longest_path: Vec<usize> = dfs(index, graph);
    let pokechain: Vec<String> = longest_path.iter().map(|x| pokedex[*x].clone()).collect();
    
    return pokechain;
}

fn dfs(start_index: usize, graph: Vec<Node>) -> Vec<usize> {
    let mut longest_path: Vec<usize> = Vec::new();
    let mut current_path: Vec<usize> = Vec::new();
    let mut up_next: Vec<usize> = Vec::new();
    let mut visited: [bool; 1000] = [false; 1000];
    let mut current_index: usize = start_index;
    let mut current_node = &graph[start_index];
    
    visited[current_index] = true;
    current_path.push(start_index);
    longest_path = current_path.clone();
    up_next.append(&mut current_node.edges.clone());

    while !up_next.is_empty() {
        current_index = up_next.last().copied().unwrap();
        if !visited[current_index] {
            current_node = &graph[current_index];
            current_path.push(current_index);
            visited[current_index] = true;

            if current_path.len() > longest_path.len() {
                longest_path = current_path.clone();
            }

            let mut edges_to_add: Vec<usize> = Vec::new();
            for  i in &current_node.edges {
                if !visited[*i] {
                    edges_to_add.push(*i)
                }
            }

            up_next.append(&mut edges_to_add);
        } else {
            current_path.pop();
            up_next.pop();
        }
    }


    return longest_path;
}
