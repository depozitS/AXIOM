
const CFG:&str = r"axiom\config\";
const CFG_AXGRAPH_BIND:&str = r"AxGraph_bind.json";

fn main() {
    println!("{}",CFG.to_owned()+CFG_AXGRAPH_BIND);
}
