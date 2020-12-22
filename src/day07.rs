use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::hash::{Hash, Hasher};

type NodeId = u64;
type Neighbor = (u32, NodeId);

#[derive(Debug)]
struct BagGraph {
    pub edges: HashMap<NodeId, Vec<Neighbor>>,
    pub redges: HashMap<NodeId, Vec<Neighbor>>,
    pub nodes: HashSet<String>,
}

impl BagGraph {
    pub fn parse(filename: &str) -> BagGraph {
        let content = fs::read_to_string(filename).unwrap();

        let mut edges = HashMap::new();
        let mut redges = HashMap::new();
        let mut nodes = HashSet::new();

        for line in content.lines() {
            let (src, neighbors) = BagGraph::parse_line(line);
            let src_id = BagGraph::hash_node(&src);
            nodes.insert(src.to_owned());
            edges.insert(src_id, neighbors.clone());
            for (weight, tgt) in neighbors {
                let redge = redges.entry(tgt).or_insert(vec![]);
                redge.push((weight, src_id));
            }
        }

        BagGraph {
            edges,
            redges,
            nodes,
        }
    }

    fn parse_line(line: &str) -> (String, Vec<Neighbor>) {
        let words: Vec<_> = line.split_ascii_whitespace().collect();

        let src = format!("{} {}", words[0], words[1]);
        let mut edges = vec![];

        for idx in (4..words.len() - 1).step_by(4) {
            if words[idx] != "no" {
                let weight: u32 = words[idx].parse().unwrap();
                let tgt = BagGraph::hash_node(
                    &format!("{} {}", words[idx + 1], words[idx + 2]).to_string(),
                );
                edges.push((weight, tgt));
            }
        }

        (src.to_string(), edges)
    }

    pub fn hash_node(node_id: &String) -> NodeId {
        let mut hasher = DefaultHasher::new();
        node_id.hash(&mut hasher);
        hasher.finish()
    }

    pub fn traverse(&self, from: NodeId) -> u32 {
        if !self.edges.contains_key(&from) {
            return 0;
        }
        let mut acc = 0;
        for (weight, neighbor) in &self.edges[&from] {
            acc += weight * (1 + self.traverse(*neighbor));
        }
        acc
    }
}

pub fn run() {
    part1();
    part2();
}

pub fn part1() {
    let graph = BagGraph::parse("inputs/day7.txt");

    let mut visited: HashSet<NodeId> = HashSet::new();
    let mut fringe: HashSet<NodeId> = HashSet::new();
    fringe.insert(BagGraph::hash_node(&"shiny gold".to_owned()));

    while fringe.len() > 0 {
        let mut next_fringe = HashSet::new();
        for id in fringe {
            if !graph.redges.contains_key(&id) {
                continue;
            }
            for (_, neighbor) in &graph.redges[&id] {
                next_fringe.insert(*neighbor);
                visited.insert(*neighbor);
            }
        }
        fringe = next_fringe;
    }

    println!("day07.part1.solution = {}", visited.len());
}

pub fn part2() {
    let graph = BagGraph::parse("inputs/day7.txt");
    let src = BagGraph::hash_node(&"shiny gold".to_owned());

    println!("day07.part2.solution = {}", graph.traverse(src));
}
