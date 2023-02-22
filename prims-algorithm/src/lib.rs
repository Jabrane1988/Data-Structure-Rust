use std::cmp::Reverse;
use std::collections::{BTreeMap, BinaryHeap};

type Graph<V, E> = BTreeMap<V, BTreeMap<V, E>>;

fn add_edge<V: Ord + Copy, E: Ord + Copy>(graph: &mut Graph<V, E>, v1: V, v2: V, wt: E) {
    graph.entry(v1).or_insert_with(BTreeMap::new).insert(v2, wt);
    graph.entry(v2).or_insert_with(BTreeMap::new).insert(v1, wt);
}

pub fn prim<V: Ord + Copy, E: Ord + Copy>(graph: &Graph<V, E>, start: V) -> Graph<V, E> {
    let mut mst = Graph::new();
    let mut visited = BinaryHeap::new();

    mst.insert(start, BTreeMap::new());

    for (v, wt) in &graph[&start] {
        visited.push(Reverse((*wt, v, start)));
    }

    while let Some(Reverse((wt, vt, prev))) = visited.pop() {
        if mst.contains_key(vt) {
            continue;
        }

        add_edge(&mut mst, prev, *vt, wt);

        for (v, wt) in &graph[vt] {
            if !mst.contains_key(v) {
                visited.push(Reverse((*wt, v, *vt)));
            }
        }
    }
    mst
}
