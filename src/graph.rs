use std::collections::HashMap;

type NodeMap<T> = HashMap<String, T>;

struct BinaryNode<D> {
    id: String,
    children: [String; 2],
    data: D,
}
