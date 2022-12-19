use anyhow::Error;
use regex::Regex;
use std::collections::HashMap;

fn get_node_index(node_indices: &mut HashMap<String, u8>, name: String) -> u8 {
    let next_index = u8::try_from(node_indices.len()).unwrap();
    let index = node_indices.entry(name).or_insert(next_index);
    *index
}

#[derive(Debug)]
struct Node {
    idx: u8,
    flow_rate: u8,
    edges: Vec<u8>,
}

#[derive(Debug)]
enum PathElem {
    // Travel to a valve (takes 1 minute)
    // Must be accessible from the current valve
    TravelTo(u8),
    // Open a value (takes 1 minutes)
    // Must be the current valve and not already open
    Open(u8),
}

fn main() -> Result<(), Error> {
    let re = Regex::new(
        r#"Valve (\S+) has flow rate=(\d+); tunnels? leads? to valves? ([^,]+)(?:, ([^,]+))*"#,
    )?;
    let mut node_indices = HashMap::new();

    let mut nodes = std::io::stdin()
        .lines()
        .map(|line| {
            let line = line?;
            let captures = re
                .captures(&line)
                .ok_or_else(|| Error::msg("Line did not match regex"))?;

            let idx = get_node_index(&mut node_indices, captures[1].to_string());
            let flow_rate = captures[2].parse()?;
            let edges = captures
                .iter()
                .skip(3)
                .flatten()
                .map(|m| get_node_index(&mut node_indices, m.as_str().to_string()))
                .collect();

            Ok(Node {
                idx,
                flow_rate,
                edges,
            })
        })
        .collect::<Result<Vec<_>, Error>>()?;

    nodes.sort_by_key(|node| node.idx);

    println!("{nodes:?}");
    Ok(())
}
