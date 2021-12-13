use std::collections::HashMap;

struct Graph {
    adj_list: HashMap<&'static str, Vec<&'static str>>,
}

impl Graph {
    fn new() -> Self {
        Self {
            adj_list: HashMap::new(),
        }
    }

    fn add_edge(&mut self, a: &'static str, b: &'static str) {
        self.adj_list.entry(a).or_insert(vec![]).push(b);
        self.adj_list.entry(b).or_insert(vec![]).push(a);
    }

    fn get_adj(&self, v: &'static str) -> &Vec<&'static str> {
        self.adj_list.get(v).unwrap()
    }

    fn get_vertices(&self) -> Vec<&'static str> {
        self.adj_list.keys().map(|&x| x).collect()
    }
}

fn can_revisit(s: &'static str) -> bool {
    s.chars().all(|c| c.is_uppercase())
}

fn dfs(graph: &Graph, visited: &mut HashMap<&str, bool>, v: &'static str, count: &mut u32) {
    if v == "end" {
        *count += 1;
        return;
    }

    graph.get_adj(v).iter().for_each(|x| {
        if can_revisit(x) || !*visited.get(x).unwrap() {
            visited.insert(x, true);
            dfs(graph, visited, x, count);
            visited.insert(x, false);
        }
    });
}

fn dfs2(graph: &Graph, visited: &mut HashMap<&str, u32>, v: &'static str, count: &mut u32) {
    if v == "end" {
        *count += 1;
        return;
    }

    graph.get_adj(v).iter().for_each(|x| {
        if can_revisit(x) {
            dfs2(graph, visited, x, count);
            return;
        }

        // Small cave
        let revisited = visited.values().any(|c| *c == 2);
        let visits = *visited.get(x).unwrap();
        if (!revisited && visits < 2) || visits < 1 {
            visited.entry(x).and_modify(|c| *c += 1);
            dfs2(graph, visited, x, count);
            visited.entry(x).and_modify(|c| *c -= 1);
        }
    });
}

fn first_puzzle() -> u32 {
    let mut graph = Graph::new();
    include_str!("in.txt").lines().for_each(|l| {
        let (a, b) = l.trim().split_once("-").unwrap();
        graph.add_edge(a, b);
    });

    let mut visited: HashMap<_, _> = graph
        .get_vertices()
        .into_iter()
        .map(|x| (x, false))
        .collect();
    let mut count = 0;
    visited.insert("start", true);
    dfs(&graph, &mut visited, "start", &mut count);

    count
}

fn second_puzzle() -> u32 {
    let mut graph = Graph::new();
    include_str!("in.txt").lines().for_each(|l| {
        let (a, b) = l.trim().split_once("-").unwrap();
        graph.add_edge(a, b);
    });

    let mut visited: HashMap<_, _> = graph.get_vertices().into_iter().map(|x| (x, 0)).collect();
    let mut count = 0;
    visited.insert("start", 3);
    dfs2(&graph, &mut visited, "start", &mut count);

    count
}

#[test]
fn test_first_puzzle() {
    println!("Day 12 (1): {}", first_puzzle())
}

#[test]
fn test_second_puzzle() {
    println!("Day 12 (2): {}", second_puzzle());
}
