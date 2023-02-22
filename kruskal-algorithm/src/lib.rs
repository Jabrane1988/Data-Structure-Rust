use disjoint_sets::UnionFind;

type Node = usize;
type Weight = usize;

struct Edge {
    dest: Node,
    weight: Weight,
}

type Graph = Vec<Vec<Edge>>;

fn edges_by_weight(graph: &Graph) -> Vec<(Node, Node, Weight)> {
    let mut edges = vec![];

    for (src, dest) in graph.iter().enumerate() {
        for edge in dest {
            edges.push((src, edge.dest, edge.weight));
        }
    }

    edges.sort_by_key(|&(_,_, weight)|weight);
    edges
}

fn mst(graph: &Graph) -> Vec<(Node, Node)> {
    let mut result = vec![];
    let mut union_find = UnionFind::new(graph.len());

    for (src, dest, _) in edges_by_weight(graph) {
        if !union_find.equiv(src, dest) {
            union_find.union(src, dest);
            result.push((src, dest));
        }
    }


    result
}


