use std::collections::HashMap;

pub struct Graph <VId, E = (), V = ()>{
    vertices: HashMap<VId, V>,
    adjacency: HashMap<VId, Vec<(VId, E)>>,
}

impl<VId, E, V> Graph<VId, E, V>
where
    VId: Eq + Hash,
    V: Hash,
{
    pub fn new() -> Graph<VId, E, V>{
        Graph { vertices: HashMap::new(), adjacency: HashMap::new()}
    }

    pub fn push_vertex(self: &mut Graph<VId, E, V>, vid: VId, vertex: v){
        self.vertices.insert(vid, vertex);
    }

    pub fn push_edge(self: &mut Self, from: VId, to: VId, edge: E){
        let adjacent_to_from = self.adjacency.entry(from).or_default();
        adjacency_to_from.push((to, edge));
    }
}

