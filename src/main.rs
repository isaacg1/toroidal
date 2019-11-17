use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Vertex(u64);
type Edge = (Vertex, Vertex);
type Graph = Vec<Edge>;

fn full_genus(graph: &Graph) -> usize {
    componenets(graph).iter().map(|g| genus(g)).sum()
}
fn genus(graph: &Graph) -> usize {
    #[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, PartialOrd, Ord)]
    struct HalfEdge(usize);
    let mut edge_pairing: HashMap<HalfEdge, HalfEdge> = HashMap::new();
    let mut vertex_groups: HashMap<&Vertex, Vec<HalfEdge>> = HashMap::new();
    let mut i = 0;
    for edge in graph {
        let (vert1, vert2) = edge;
        let half1 = HalfEdge(i);
        i += 1;
        let half2 = HalfEdge(i);
        i += 1;
        edge_pairing.insert(half1, half2);
        edge_pairing.insert(half2, half1);
        vertex_groups.entry(vert1).or_insert(vec![]).push(half1);
        vertex_groups.entry(vert2).or_insert(vec![]).push(half2);
    }
    let mut vertex_groups: Vec<Vec<HalfEdge>> = vertex_groups.values().cloned().collect();
    vertex_groups.sort();
    println!("{:?}", edge_pairing);
    println!("{:?}", vertex_groups);

    let mut max_faces = 0;
    let num_rotations = vertex_groups
        .iter()
        .map(|v| v.len() - 1)
        .map(|n| {
            let mut prod = 1;
            for i in 0..n {
                prod *= i + 1;
            }
            prod
        })
        .fold(1, |p, n| p * n);
    println!("\nNum rotations: {}\n", num_rotations);
    for rotation_index in 0..num_rotations {
        let mut working_index = rotation_index;
        let mut rotation: HashMap<HalfEdge, HalfEdge> = HashMap::new();
        let mut pretty_rotation: Vec<Vec<HalfEdge>> = vec![];
        for group in &vertex_groups {
            let mut removal_group = group.clone();
            let start = removal_group.pop().unwrap();
            let mut from = start;
            let mut pretty_group = vec![from];
            while !removal_group.is_empty() {
                let index = working_index % removal_group.len();
                working_index /= removal_group.len();
                let to = removal_group.swap_remove(index);
                rotation.insert(from, to);
                pretty_group.push(to);
                from = to;
            }
            rotation.insert(from, start);
            pretty_rotation.push(pretty_group);
        }
        let mut seen_on_face: HashSet<&HalfEdge> = HashSet::new();
        let mut num_faces = 0;
        for start_halfedge in edge_pairing.keys() {
            if !seen_on_face.contains(start_halfedge) {
                num_faces += 1;
                let mut current_halfedge = start_halfedge;
                loop {
                    seen_on_face.insert(current_halfedge);
                    let pair_halfedge = &edge_pairing[current_halfedge];
                    current_halfedge = &rotation[pair_halfedge];
                    if current_halfedge == start_halfedge {
                        break;
                    }
                }
            }
        }
        if num_faces > max_faces {
            max_faces = num_faces;
            let euler_characteristic: isize =
                vertex_groups.len() as isize - graph.len() as isize + max_faces as isize;
            let genus_num: isize = 1 - euler_characteristic / 2;
            println!(
                "Faces: {}, Genus <= {} on rotation {}",
                max_faces, genus_num, rotation_index
            );
            println!("{:?}\n", pretty_rotation);
        }
        if rotation_index % 1e7 as usize == 0 {
            println!("Faces: {} <= {} on rotation {}", num_faces, max_faces, rotation_index);
        }
    }
    let euler_characteristic: isize =
        vertex_groups.len() as isize - graph.len() as isize + max_faces as isize;
    let genus_num: isize = 1 - euler_characteristic / 2;
    assert!(genus_num >= 0);
    genus_num as usize
}

fn componenets(graph: &Graph) -> Vec<Graph> {
    let mut graphs: Vec<(Graph, HashSet<Vertex>)> = vec![];
    for edge in graph {
        let (vert1, vert2) = edge;
        let g_index1 = graphs.iter().position(|(_g, h)| h.contains(&vert1));
        let g_index2 = graphs.iter().position(|(_g, h)| h.contains(&vert2));
        match (g_index1, g_index2) {
            (Some(i1), Some(i2)) => {
                if i1 != i2 {
                    let (mut graph1, mut vs1) = graphs.remove(i1);
                    let new_i2 = if i1 < i2 { i2 - 1 } else { i2 };
                    let (graph2, vs2) = graphs.remove(new_i2);
                    graph1.extend(graph2);
                    vs1.extend(vs2);
                    graphs.push((graph1, vs1));
                } else {
                    graphs[i1].0.push(edge.clone());
                }
            }
            (Some(i1), None) => {
                graphs[i1].0.push(edge.clone());
                graphs[i1].1.insert(vert2.clone());
            }
            (None, Some(i2)) => {
                graphs[i2].0.push(edge.clone());
                graphs[i2].1.insert(vert1.clone());
            }
            (None, None) => {
                let edges = vec![edge.clone()];
                let mut vs = HashSet::new();
                vs.insert(vert1.clone());
                vs.insert(vert2.clone());
                graphs.push((edges, vs));
            }
        }
    }
    graphs.into_iter().map(|(g, _h)| g).collect()
}

fn main() {
    let graph = vec![
        (Vertex(0), Vertex(5)),
        (Vertex(0), Vertex(6)),
        (Vertex(0), Vertex(7)),
        (Vertex(0), Vertex(8)),
        (Vertex(1), Vertex(5)),
        (Vertex(1), Vertex(6)),
        (Vertex(1), Vertex(7)),
        (Vertex(1), Vertex(8)),
        (Vertex(2), Vertex(5)),
        (Vertex(2), Vertex(6)),
        (Vertex(2), Vertex(7)),
        (Vertex(2), Vertex(8)),
        (Vertex(3), Vertex(5)),
        (Vertex(3), Vertex(6)),
        (Vertex(3), Vertex(7)),
        (Vertex(3), Vertex(8)),
        (Vertex(4), Vertex(5)),
        (Vertex(4), Vertex(6)),
        (Vertex(4), Vertex(7)),
        (Vertex(4), Vertex(8)),
    ];
    let result = full_genus(&graph);
    println!("Result: {}", result);
}
