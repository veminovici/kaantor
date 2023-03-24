pub struct Node<K> {
    key: K,
    ns: Vec<K>,
}

impl<K> Node<K> {
    pub fn new(key: K) -> Self {
        Self { key, ns: vec![] }
    }

    pub fn with_child(key: K, cid: K) -> Self {
        Self { key, ns: vec![cid] }
    }

    pub fn key(&self) -> &K {
        &self.key
    }

    pub fn add_edge_to(&mut self, key: K) {
        self.ns.push(key)
    }

    pub fn neighbours(&self) -> impl Iterator<Item = &K> {
        self.ns.iter()
    }
}

impl<K> From<K> for Node<K> {
    fn from(key: K) -> Self {
        Node::new(key)
    }
}
