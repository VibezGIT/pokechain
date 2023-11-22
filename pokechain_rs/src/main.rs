use std::fs;
use std::collections::HashMap;

fn main() {
    let file = fs::read_to_string("./pokedex.json").expect("Unable to read file");
    let json: serde_json::Value = serde_json::from_str(&file).expect("JSON was not properly formatted.");
    let pokedex = json.as_array().unwrap();
    // let length = pokedex.len();
    let length = 64;

    let mut nodes: HashMap<char, HashMap<char, u32>> = HashMap::new();
    let mut remainder: HashMap<char, u32> = HashMap::new();

    for i in 0..length {
        let start: char = pokedex[i].as_str().unwrap().to_ascii_lowercase().chars().next().unwrap();
        let end: char = pokedex[i].as_str().unwrap().to_ascii_lowercase().chars().last().unwrap();

        if nodes.contains_key(&start) {
            if nodes[&start].contains_key(&end) {
                let mut new_map = nodes[&start].clone();
                new_map.insert(end, nodes[&start][&end] + 1);
                nodes.insert(start, new_map);
            } else {
                let mut new_map = nodes[&start].clone();
                new_map.insert(end, 1);
                nodes.insert(start, new_map);
            }
        } else {
            let mut new_map = HashMap::new();
            new_map.insert(end, 1);
            nodes.insert(start, new_map);
        }

        if remainder.contains_key(&start) {
            remainder.insert(start, remainder[&start] + 1);
        } else {
            remainder.insert(start, 1);
        }
    }

    println!("{:?}", dft(&'b', &nodes, &Vec::new(), &remainder));
}

fn dft(node: &char, nodes: &HashMap<char, HashMap<char, u32>>, chain: &Vec<char>, remainder: &HashMap<char, u32>) -> Vec<char> {

    let mut new_chain = chain.to_vec();
    new_chain.push(*node);
    let mut longest_chain = new_chain.to_vec();

    if nodes.contains_key(node) {
        for edge in nodes[&node].keys() {
            let mut new_rem = remainder.clone();
            if new_rem.contains_key(edge) {
                if new_rem[edge] == 1 {
                    new_rem.remove(edge);
                } else {
                    new_rem.insert(*edge, new_rem[edge] - 1);
                }
                let result = dft(edge, &nodes, &new_chain, &new_rem);
                
                if result.len() > longest_chain.len() {
                    longest_chain = result.to_vec();
                }
            }
        }
    }

    return longest_chain;
}