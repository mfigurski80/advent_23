struct Node<T> {
    id: usize,
    children: Option<Box<Vec<Node>>>,
    data: T,
}

fn do_dfs<T>(node: &Node<T>, action: &Fn(&Node<T>)) {
    for child in node.children.iter() {
        action(child);
        do_dfs(child);
    }
}
