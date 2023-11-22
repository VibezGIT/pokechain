use std::fs;
use std::collections::HashMap;


fn create_graph(pokedex: &Vec<String>) -> HashMap<char, HashMap<char, usize>> {
  let mut graph: HashMap<char, HashMap<char, usize>> = HashMap::new();

  for i in 0..pokedex.len() {
    let start = pokedex[i].chars().next().unwrap();
    let end = pokedex[i].chars().last().unwrap();

    match graph.get_mut(&start) {
      None => {
        graph.insert(start, HashMap::new());
      }
      _ => ()
    }

    let mut edges = graph.get_mut(&start).unwrap();
    match edges.get_mut(&end) {
      None => {
        edges.insert(end, 1);
      }
      Some(v) => {
        let new_value = *v + 1;
        edges.insert(end, new_value);
      }
    }
  }

  return graph;
}

fn create_remainder(pokedex: &Vec<String>) -> HashMap<char, usize> {
  let mut remainder: HashMap<char, usize> = HashMap::new();

  for i in 0..pokedex.len() {
    let start = pokedex[i].chars().next().unwrap(); 
    match remainder.get_mut(&start) {
      None => {
        remainder.insert(start, 1);
      }
      Some(v) => {
        let new_value = *v + 1;
        remainder.insert(start, new_value);
      }
    }
  }

  return remainder;
}

fn main() {
  let pokedex_length = 898; //MAX 898

  let file = fs::read_to_string("pokedex.json").expect("Unable to read file");
  let pokedex: Vec<String> = serde_json::from_str(&file).expect("JSON was not properly formatted.");  
  let graph = create_graph(&pokedex[0..pokedex_length].to_vec());
  let remainder = create_remainder(&pokedex[0..pokedex_length].to_vec());

  println!("{:?}", graph);
}
