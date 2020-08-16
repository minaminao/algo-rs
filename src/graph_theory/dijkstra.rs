use std::cmp::Reverse;
use std::collections::BinaryHeap;

const INF_DIST: usize = std::usize::MAX;
type Graph = Vec<Vec<(usize, usize, usize)>>;

/// Dijkstra's algorithm
pub fn dijkstra(g: &Graph, start: usize) -> Vec<usize> {
    let n = g.len();
    let mut vis = vec![false; n];
    let mut dist = vec![INF_DIST; n];
    let mut heap = BinaryHeap::new();

    dist[start] = 0;
    heap.push(Reverse((0, start)));

    while let Some(Reverse((cost, v))) = heap.pop() {
        if dist[v] < cost {
            continue;
        }
        vis[v] = true;
        for e in &g[v] {
            let (_, d, w) = &e;
            if vis[*d] {
                continue;
            }
            if dist[*d] > dist[v] + w {
                dist[*d] = dist[v] + w;
                heap.push(Reverse((dist[*d], *d)));
            }
        }
    }
    dist
}
