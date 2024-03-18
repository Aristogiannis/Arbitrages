use petgraph::graph::Graph;
use serde_json::Value;
use petgraph::dot::Dot;
use petgraph::algo::find_negative_cycle;
use petgraph::prelude::*;

pub fn identify_arbitrage(data: String) {

    let json: Vec<Value> = serde_json::from_str(&data)
        .expect("JSON does not have correct format.");

    let mut graph = Graph::new_undirected();
    let mut nodes = std::collections::HashMap::new();

for entry in json {
    if let Some(symbol) = entry.get("symbol").and_then(|v| v.as_str()) {
        if let Some(price_str) = entry.get("price").and_then(|v| v.as_str()) {
            if let Ok(price) = price_str.parse::<f64>() {
                // Extract the currency symbols from the symbol string.
                let (left, right) = symbol.split_at(symbol.len() - 3);

                // Retrieve or insert the nodes for the currencies.
                let node_a = *nodes.entry(left.to_string()).or_insert_with(|| graph.add_node(left.to_string()));
                let node_b = *nodes.entry(right.to_string()).or_insert_with(|| graph.add_node(right.to_string()));

                // Add an edge with the exchange price as the weight.
                graph.add_edge(node_a, node_b, -1.0*price.log2());
            }
        }
    }
}
let path = find_negative_cycle(&graph, NodeIndex::new(0));
// Debug print to visualize the graph structure.
//dbg!(graph);
println!("{}", Dot::new(&graph));

if let Some(p) = &path {
    println!("Negative cycle found: {:?}", p);

} else {
    println!("No negative cycle found");
}
}