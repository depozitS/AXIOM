
const CFG:&str = r"axiom\config\";
const CFG_AXGRAPH_BIND:&str = r"AxGraph_bind.json";

mod file_io;
mod json;



fn main() {
    println!("{}",CFG.to_owned()+CFG_AXGRAPH_BIND);
}
