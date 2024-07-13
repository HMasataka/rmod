mod id;
mod node;

fn main() {
    let node = node::Node::new(id::ID::new("1"), "Node 1");

    println!("Hello, module: {:?}", node);
}
