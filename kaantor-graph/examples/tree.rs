use kaantor_graph::{Node, Tree};

fn main() {
    let mut tree = Tree::new(1);

    let mut n1 = Node::new(1);
    n1.add_edge_to(2);
    n1.add_edge_to(3);
    tree.add_node(n1);

    let mut n2 = Node::new(2);
    n2.add_edge_to(4);
    tree.add_node(n2);

    let n3 = Node::new(3);
    tree.add_node(n3);

    let n4 = Node::new(4);
    tree.add_node(n4);

    tree.pretty_print("SPANNING_TREE");
}
