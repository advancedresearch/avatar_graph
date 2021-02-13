//! # Avatar Graph
//!
//! A library for Avatar Graphs.
//!
//! ![avatar5-01.png](https://raw.githubusercontent.com/advancedresearch/avatar_graph/master/images/avatar5-01.png)
//! ![avatar8-02.png](https://raw.githubusercontent.com/advancedresearch/avatar_graph/master/images/avatar8-02.png)
//! ![avatar3-01.png](https://raw.githubusercontent.com/advancedresearch/avatar_graph/master/images/avatar3-01.png)
//!
//! *An Avatar Graph is a special kind of graph that is useful for mathematical programming.*
//!
//! An Avatar Graph is an sub-type of graphs
//! which is used as a theorem proving technique/tool in [Path Semantics](https://github.com/advancedresearch/path_semantics).
//!
//! For more information, see the paper [Avatar Graphs](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/avatar-graphs.pdf).
//!
//! ### Example: Avatar Graphs and Hypercubes
//!
//! Here is an example of theorem proving with Avatar Graphs.
//!
//! Conjecture: Any Avatar Graphs containing only core candidates is
//! isomorphic to some hypercube.
//!
//! If this conjecture is correct, then the number of nodes required
//! to construct a "filled" Avatar Graph is:
//!
//! `2^n` where `n = 0, 1, 2, ...`.
//!
//! This conjecture has a counter-proof:
//!
//! ![wagner.png](https://raw.githubusercontent.com/advancedresearch/avatar_graph/master/images/wagner.png)
//!
//! This is called a "Wagner graph". See [wikipedia article](https://en.wikipedia.org/wiki/Wagner_graph) for more information.
//!
//! It is easier to see that this graph is not isomorphic to a cube, drawn this way:
//!
//! ![wagner-mobius.png](https://raw.githubusercontent.com/advancedresearch/avatar_graph/master/images/wagner-mobius.png)
//!
//! However, even the conjecture is wrong, what about the `2^n` law?
//! A Wagner graph has 8 nodes, which is `2^3`.
//!
//! Conjecture: The number of nodes in every filled Avatar Graph is `2^n` for some `n`.
//!
//! This conjecture has a counter-proof:
//!
//! ![avatar6-03.png](https://raw.githubusercontent.com/advancedresearch/avatar_graph/master/images/avatar6-03.png)
//!
//! ### Introduction to Avatar Graphs
//!
//! ![avatar16-01.png](https://raw.githubusercontent.com/advancedresearch/avatar_graph/master/images/avatar16-01.png)
//!
//! *A hypercube of 4 dimensions. Any hypercube contains only nodes called "core candidates".*
//!
//! A more practical example for programmers: [The New Type Idiom in Rust](https://doc.rust-lang.org/rust-by-example/generics/new_types.html).
//!
//! - Why does this technique work?
//! - How can one prove that such techniques work in general in mathematics?
//! - How can such techniques be generalized?
//!
//! It turns out that there is a mathematical pattern behind such generalizations.
//! What these patterns have in common, is that they deal with some kind of
//! encapsulation of information of objects such that they "play a different role",
//! yet they "integrate information" with the original object.
//!
//! At abstract level, this pattern takes the form of an Avatar Graph.
//!
//! Avatar Graphs are first and foremost a tool for insight and inspiration.
//! They are usually not needed to be represented explicitly.
//! This means Avatar Graphs can tell something about mathematical theories
//! that is not clear, seen from within the theory.
//!
//! ### Diagrams of Avatar Graphs
//!
//! The original mathematical model is called the "core self" or just "core" in an Avatar Graph.
//! Avatar Graphs are used to study symmetries of core selfs from avatar extensions.
//!
//! - black node: Candidate for core self
//! - white node: N-avatar where where the smallest possible `N` is greater than zero
//! - black edge: Directed edge depending on choice of core self candidate
//! - grey/dashed edge: A unique edge from a core self candidate to highest avatar
//!
//! ![avatar7-01.png](https://raw.githubusercontent.com/advancedresearch/avatar_graph/master/images/avatar7-01.png)
//!
//! ### Unique Highest Avatar
//!
//! Each candidate core has an associated unique highest avatar,
//! due to removal of all irrelevant structure from the graph.
//!
//! This is because adding new n-avatars at same level is trivial,
//! so it is only interesting to study graphs whith a unique highest avatar.
//!
//! The choice of highest avatar might depend on order of nodes.
//! This limitation is mostly irrelevant for most interesting graphs.
//! However, it greatly simplifies the calculation of avatar distances.
//!
//! ### Avatar Distances
//!
//! The `n` in n-avatar is also a kind of distance in the graph, called "avatar distance".
//!
//! An avatar distance counts the number of paths to the core self.
//!
//! It is constructed by first finding the shortest distance for all nodes,
//! and then "relaxed" by summing avatar distances of children.
//! The choice of children might depend on the order of nodes.
//!
//! The exact value of an avatar distance does not matter,
//! except as a mean to find the highest avatar relative to a core candidate.
//!
//! Think of it of as a somewhat complex puzzle that needs to be solved,
//! and the avatar distances is just a technique for solving the puzzle.
//!
//! Avatar distances have different semantics than the shortest distance.
//! Two nodes with equal shortest distance can have different avatar distances.
//!
//! ### Avatar Connectivity
//!
//! Avatars at the same level are not allowed to communicate.
//!
//! A 1-avatar can only pass information to the core self.
//!
//! A 2-avatar can only pass information to two 1-avatars.
//!
//! A 3-avatar can pass information to 3x1-avatars or 2+1-avatars.
//!
//! An n-avatar can pass information to any partition of lower avatars.
//!
//! ### Non-Contractability
//!
//! When an avatar has only one child relative to shortest distance, it is contractible,
//! which is not permitted relative to a core candidate.
//!
//! The core self is not counted as a child for contractability.
//! This means 1-avatars have zero children, therefore not one child.
//! One can also think about it as 1-avatars being an exception from the rule.
//!
//! ### Semi-Contractibility
//!
//! Since the non-contractibility rule uses shortest distance instead of avatar distances,
//! there are some cases where avatars are semi-contractible.
//!
//! ![avatar8-04.png](https://raw.githubusercontent.com/advancedresearch/avatar_graph/master/images/avatar8-04.png)
//!
//! *Example of a graph with semi-contractibility.*
//!
//! Semi-contractibility happens when avatars are contractible by avatar distance but not by shortest distance.
//!
//! Semi-contractibility means that two avatars at same level need to agree on how to integrate information.
//! They are using each other as children, depending on the order of nodes in the graph.
//!
//! This property is allowed since otherwise the Avatar Graphs would not be symmetric
//! in cases where the topology contains features similar to a Möbius strip.
//!
//! ### Universal Reachability
//!
//! All nodes in the graph must be reachable when
//! walking from the highest avatar to the core candidate,
//! without moving further away than the distance given by the shortest path.
//!
//! With other words, there exists some path from the highest avatar to any node,
//! while either moving closer, or preserving the distance, to the core candidate.
//!
//! Another way of thinking about this property, is that for a well-chosen path
//! from the highest avatar to the core, one can visit any node.
//!
//! This property is beneficial in systems where you want to have choices,
//! but you also want to avoid regression.

/// Represents a node in the graph.
#[derive(Debug, Clone)]
pub struct Node {
    /// Whether the node is a core.
    pub core: bool,
    /// A unique edge to the highest avatar.
    pub uniq: Option<usize>,
}

impl Node {
    /// Creates a new node with no unique connection.
    pub fn new(core: bool) -> Node {
        Node {
            core, uniq: None
        }
    }
}

/// Represents an Avatar Graph.
#[derive(Debug, Clone)]
pub struct Graph {
    /// Stores nodes.
    pub nodes: Vec<Node>,
    /// Stores edges between nodes.
    pub edges: Vec<(usize, usize)>,
}

impl Graph {
    /// Creates a new empty graph.
    pub fn new() -> Graph {
        Graph {
            nodes: vec![],
            edges: vec![],
        }
    }

    /// Adds a new node.
    pub fn add_node(&mut self, node: Node) -> usize {
        let id = self.nodes.len();
        self.nodes.push(node);
        id
    }

    /// Adds a new edge.
    pub fn add_edge(&mut self, a: usize, b: usize) -> usize {
        let min = a.min(b);
        let max = a.max(b);
        let id = self.edges.len();
        for i in 0..self.edges.len() {
            if self.edges[i] == (min, max) {return i};
        }
        self.edges.push((min, max));
        id
    }

    /// Counts the number of cores.
    pub fn cores(&self) -> usize {
        let mut sum = 0;
        for node in &self.nodes {
            if node.core {sum += 1}
        }
        sum
    }

    /// Counts the number of non-cores.
    pub fn non_cores(&self) -> usize {
        self.nodes.len() - self.cores()
    }

    /// Returns a list of nodes connected by edges of a node.
    pub fn edges_of(&self, node: usize) -> Vec<usize> {
        let mut res = vec![];
        for &(a, b) in &self.edges {
            if a == node {res.push(b)}
            if b == node {res.push(a)}
        }
        res
    }

    /// Counts the number of unique edges.
    pub fn unique_edges(&self) -> usize {
        let mut sum = 0;
        for node in &self.nodes {
            if node.uniq.is_some() {sum += 1}
        }
        sum
    }

    /// Counts the number of self unique edges.
    pub fn self_unique_edges(&self) -> usize {
        let mut sum = 0;
        for i in 0..self.nodes.len() {
            if let Some(j) = self.nodes[i].uniq {
                if i == j {sum += 1}
            }
        }
        sum
    }

    /// Removes all self unique edges.
    pub fn remove_self_unique_edges(&mut self) {
        for i in 0..self.nodes.len() {
            if let Some(j) = self.nodes[i].uniq {
                if i == j {self.nodes[i].uniq = None}
            }
        }
    }

    /// Counts the number of self edges.
    pub fn self_edges(&self) -> usize {
        let mut sum = 0;
        for &(a, b) in &self.edges {
            if a == b {sum += 1}
        }
        sum
    }

    /// Removes all self edges.
    pub fn remove_self_edges(&mut self) {
        for i in (0..self.edges.len()).rev() {
            let (a, b) = self.edges[i];
            if a == b {self.edges.swap_remove(i);}
        }
    }

    /// Returns a matrix representation of the graph.
    ///
    /// - `0` means there is no edge.
    /// - `1` means there is an edge.
    /// - `2` means there is a unique edge.
    /// - `3` means there is an edge and a unique edge.
    pub fn matrix(&self) -> Vec<Vec<u8>> {
        let n = self.nodes.len();
        let mut mat = vec![vec![0; n]; n];
        for i in 0..n {
            if let Some(j) = self.nodes[i].uniq {
                let min = i.min(j);
                let max = i.max(j);
                mat[min][max] = 2;
            }
        }
        for &(a, b) in &self.edges {
            mat[a][b] += 1;
        }
        mat
    }

    /// Assigns each node a distance number from a particular node.
    ///
    /// Returns `Ok` if the entire graph is connected.
    /// Returns `Err` if there is some node that is disconnected.
    pub fn distance(&self, ind: usize) -> Result<Vec<(usize, u64)>, Vec<(usize, u64)>> {
        let mut dist = vec![(ind, 0)];
        let mut nodes: Vec<usize> = (0..self.nodes.len()).filter(|&n| n != ind).collect();
        while dist.len() < self.nodes.len() {
            let mut found_any = false;
            for i in (0..nodes.len()).rev() {
                let j = nodes[i];
                let edges = self.edges_of(j);
                let mut min: Option<u64> = None;
                for &e in &edges {
                    for k in 0..dist.len() {
                        if dist[k].0 == e {
                            if min.is_none() || min.unwrap() > dist[k].1 {
                                min = Some(dist[k].1);
                            }
                        }
                    }
                }
                if min.is_some() {
                    dist.push((j, min.unwrap() + 1));
                    found_any = true;
                    nodes.swap_remove(i);
                }
            }
            if !found_any {
                dist.sort();
                return Err(dist);
            }
        }
        dist.sort();
        loop {
            let mut found_any = false;
            for i in 0..dist.len() {
                let j = dist[i].0;
                let edges = self.edges_of(j);
                for &e in &edges {
                    let k = dist.binary_search_by(|n| n.0.cmp(&e)).unwrap();
                    if dist[j].1 > dist[k].1 + 1 {
                        dist[j].1 = dist[k].1 + 1;
                        found_any = true;
                    }
                }
            }
            if !found_any {break};
        }
        Ok(dist)
    }

    /// Returns avatar distances of nodes from a core node.
    ///
    /// The avatar distance is greater or equal to shortest distance.
    /// An avatar distance for a node is the sum of avatar distances of its
    /// children, which have shorter or equal distance to the core.
    ///
    /// The avatar distance is also the number of paths to the core self.
    pub fn avatar_distance(&self, ind: usize) -> Vec<(usize, u64)> {
        let mut dist = match self.distance(ind) {
            Ok(x) => x,
            Err(x) => x,
        };
        // Order by shortest distance to enumerate children per node.
        dist.sort_by_key(|n| n.1);

        // Count children by shortest distance.
        let mut count_children = vec![0; self.nodes.len()];
        for i in 0..dist.len() {
            let j = dist[i].0;
            let edges = self.edges_of(j);
            let mut count = 0;
            for &e in &edges {
                for k in 0..i {
                    if dist[k].0 != e {continue};
                    count += 1;
                }
            }
            count_children[j] = count;
        }
        // Resort such that those with fewer children
        // get a chance at re-balancing.
        dist.sort_by(|a, b| {
            a.1.cmp(&b.1).then((-count_children[a.0]).cmp(&-count_children[b.0]))
        });

        for i in 0..dist.len() {
            let j = dist[i].0;
            let edges = self.edges_of(j);
            // Sum avatar distances of children.
            let mut sum = 0;
            for &e in &edges {
                for k in 0..i {
                    if dist[k].0 != e {continue};
                    let m = dist[k].1;
                    sum += if m == 0 {1} else {m};
                }
            }
            dist[i].1 = sum.max(dist[i].1);
        }
        dist.sort();
        dist
    }

    /// Returns a list of maximum avatars and their maximum avatar distance.
    pub fn max_avatars(&self, ind: usize) -> (u64, Vec<usize>) {
        let dist = self.avatar_distance(ind);
        let mut max = 0;
        let mut avatars = vec![];
        for &(a, n) in &dist {
            if n > max {
                avatars.clear();
                avatars.push(a);
                max = n;
            } else if n == max {
                avatars.push(a);
            }
        }
        (max, avatars)
    }

    /// Returns the nodes that are contractible.
    ///
    /// A node is contractible if it has only one children with shorter distance to core.
    /// However, nodes that are directly connected to core are not counted.
    pub fn contractible(&self, ind: usize) -> usize {
        let mut dist = match self.distance(ind) {
            Ok(x) => x,
            Err(x) => x,
        };
        // Order by shortest distance to enumerate children per node.
        dist.sort_by_key(|n| n.1);
        let mut contr = 0;
        for i in 0..dist.len() {
            let j = dist[i].0;
            let n = dist[i].1;
            let edges = self.edges_of(j);
            // Sum shortest distances of children.
            let mut count = 0;
            for &e in &edges {
                for k in (0..dist.len()).rev() {
                    if dist[k].0 != e {continue};
                    let m = dist[k].1;
                    if m == 0 || m > n {continue};
                    count += 1;
                }
            }
            if count == 1 {
                contr += 1;
            }
        }
        contr
    }

    /// Returns the contractible nodes relative to a core.
    pub fn contractibles_of(&self, ind: usize) -> Vec<usize> {
        let mut dist = match self.distance(ind) {
            Ok(x) => x,
            Err(x) => x,
        };
        // Order by shortest distance to enumerate children per node.
        dist.sort_by_key(|n| n.1);
        let mut res = vec![];
        for i in 0..dist.len() {
            let j = dist[i].0;
            let n = dist[i].1;
            let edges = self.edges_of(j);
            // Sum avatar distances of children.
            let mut count = 0;
            for &e in &edges {
                for k in (0..dist.len()).rev() {
                    if dist[k].0 != e {continue};
                    let m = dist[k].1;
                    if m == 0 || m > n {continue};
                    count += 1;
                }
            }
            if count == 1 {
                res.push(j);
            }
        }
        res
    }

    /// Swaps two nodes.
    pub fn swap(&mut self, a: usize, b: usize) {
        // Swap edges.
        for i in 0..self.edges.len() {
            let (ea, eb) = self.edges[i];
            let ea = if ea == a {b} else if ea == b {a} else {ea};
            let eb = if eb == a {b} else if eb == b {a} else {eb};
            self.edges[i] = (ea.min(eb), ea.max(eb));
        }
        // Swap unique edges.
        for i in 0..self.nodes.len() {
            if let Some(j) = self.nodes[i].uniq {
                self.nodes[i].uniq = Some(if j == a {b} else if j == b {a} else {j});
            }
        }
        // Swap nodes.
        self.nodes.swap(a, b);
    }

    /// Returns nodes that are visited when walking from `a` to `b`
    /// with decreasing shortest distance.
    ///
    //// Returns `Err` if `b` can not be reached from `a`.
    pub fn along(&self, a: usize, b: usize) -> Result<Vec<usize>, ()> {
        let dist = match self.distance(b) {
            Ok(x) => x,
            Err(_) => return Err(())
        };
        let k = dist.binary_search_by(|n| n.0.cmp(&a)).map_err(|_| ())?;
        let max_dist = dist[k].1;
        let mut at = vec![dist[k]];
        let mut i = 0;
        let mut reached = vec![false; dist.len()];
        reached[k] = true;
        loop {
            if i >= at.len() {break};
            if reached.iter().all(|&b| b) {break};
            let j = at[i].0;
            // Ignore edges of target,
            // since other edges connected to it should not be added.
            if at[i].1 != 0 {
                let edges = self.edges_of(j);
                for e in &edges {
                    if reached[*e] {continue};
                    let k = dist.binary_search_by(|n| n.0.cmp(e)).unwrap();
                    // Ignore edges that lead to longer shortest distance than start node.
                    if dist[k].1 > max_dist {continue};
                    at.push(dist[k]);
                    reached[k] = true;
                }
            }
            i += 1;
        }
        let mut nodes: Vec<usize> = at.into_iter().map(|n| n.0).collect();
        nodes.sort();
        Ok(nodes)
    }

    /// Returns `true` if all nodes are reachable from `a` to `b` when
    /// walking along the gradient of shortest distances.
    pub fn all_reachable_along(&self, a: usize, b: usize) -> bool {
        match self.along(a, b) {
            Ok(v) => v == (0..self.nodes.len()).collect::<Vec<usize>>(),
            Err(()) => false,
        }
    }

    /// Returns `true` if a graph has correct avatar connectivity.
    pub fn avatar_connectivity(&self, ind: usize) -> bool {
        let dist = self.avatar_distance(ind);
        for i in 0..dist.len() {
            let j = dist[i].0;
            let n = dist[i].1;
            let edges = self.edges_of(j);
            for &e in &edges {
                let k = dist.binary_search_by(|n| n.0.cmp(&e)).unwrap();
                let m = dist[k].1;
                if dist[k].0 == e {
                    if !match n {
                        0 => m == 1,
                        1 => m == 0 || m > 1,
                        n => m > 0 && m < n || m > n,
                    } {return false};
                }
            }
        }
        true
    }

    /// Returns a list of nodes which have wrong avatar connectivity.
    pub fn avatar_connectivity_failures_of(&self, ind: usize) -> Vec<usize> {
        let mut dist = self.avatar_distance(ind);
        dist.sort_by_key(|n| n.1);
        let mut res = vec![];
        for i in 0..dist.len() {
            let j = dist[i].0;
            if j == ind {continue};
            let n = dist[i].1;
            let edges = self.edges_of(j);
            let mut found = false;
            'outer: for &e in &edges {
                for k in 0..dist.len() {
                    let m = dist[k].1;
                    if dist[k].0 == e {
                        if !match n {
                            0 => m == 1,
                            1 => m == 0 || m > 1,
                            n => m > 0 && m < n || m > n,
                        } {
                            found = true;
                            break 'outer;
                        }
                    }
                }
            }
            if found {
                res.push(j);
            }
        }
        res
    }

    /// Returns `true` if the graph is an Avatar Graph seen from a core.
    pub fn is_avatar_graph(&self, ind: usize) -> bool {
        // There can be no contractible nodes.
        if self.contractible(ind) != 0 {return false};
        // The whole graph must be connected.
        if self.distance(ind).is_err() {return false};
        // There must exist only one max avatar.
        let max_avatars = self.max_avatars(ind);
        if max_avatars.1.len() != 1 {return false};
        // All nodes must be reachable when walking from max avatar to the core.
        if !self.all_reachable_along(max_avatars.1[0], ind) {return false};
        // Nodes must follow rules for avatar connectivity.
        if !self.avatar_connectivity(ind) {return false};
        true
    }

    /// Marks all nodes as core that can be a core,
    /// unmarks all nodes that can not be a core.
    pub fn corify(&mut self) {
        for i in 0..self.nodes.len() {
            if self.is_avatar_graph(i) {
                self.nodes[i].core = true;
                self.nodes[i].uniq = Some(self.max_avatars(i).1[0])
            } else {
                self.nodes[i].core = false;
                self.nodes[i].uniq = None;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_graph() {
        let mut g = Graph::new();
        let a = g.add_node(Node::new(true));
        let b = g.add_node(Node::new(false));
        g.add_edge(a, b);
        assert_eq!(g.nodes.len(), 2);
        assert_eq!(g.edges.len(), 1);
        assert_eq!(g.cores(), 1);
        assert_eq!(g.non_cores(), 1);
        assert_eq!(g.edges_of(a), vec![b]);
        assert_eq!(g.edges_of(b), vec![a]);
        assert_eq!(g.self_edges(), 0);
        assert_eq!(g.matrix(), vec![
            vec![0, 1],
            vec![0, 0]
        ]);
        assert_eq!(g.unique_edges(), 0);
    }

    #[test]
    fn remove_self_edges() {
        let mut g = Graph::new();
        let a = g.add_node(Node::new(true));
        g.add_edge(a, a);
        assert_eq!(g.self_edges(), 1);
        g.remove_self_edges();
        assert_eq!(g.self_edges(), 0);
        assert_eq!(g.matrix(), vec![
            vec![0]
        ]);
        assert_eq!(g.unique_edges(), 0);
    }

    #[test]
    fn unique_edge() {
        let mut g = Graph::new();
        let a = g.add_node(Node::new(true));
        let b = g.add_node(Node::new(false));
        assert_eq!(g.matrix(), vec![
            vec![0, 0],
            vec![0, 0]
        ]);
        assert_eq!(g.unique_edges(), 0);
        g.nodes[a].uniq = Some(b);
        assert_eq!(g.unique_edges(), 1);
        assert_eq!(g.matrix(), vec![
            vec![0, 2],
            vec![0, 0]
        ]);
        g.add_edge(a, b);
        assert_eq!(g.matrix(), vec![
            vec![0, 3],
            vec![0, 0]
        ]);
        assert_eq!(g.unique_edges(), 1);
    }

    #[test]
    fn self_unique_edge() {
        let mut g = Graph::new();
        let a = g.add_node(Node::new(true));
        assert_eq!(g.self_unique_edges(), 0);
        g.nodes[a].uniq = Some(a);
        assert_eq!(g.self_unique_edges(), 1);
        g.remove_self_unique_edges();
        assert_eq!(g.self_unique_edges(), 0);
    }

    #[test]
    fn order() {
        let mut g = Graph::new();
        let a = g.add_node(Node::new(true));
        let b = g.add_node(Node::new(false));
        assert_eq!(g.distance(a), Err(vec![(a, 0)]));
        assert_eq!(g.distance(b), Err(vec![(b, 0)]));
        g.add_edge(a, b);
        assert_eq!(g.distance(a), Ok(vec![(a, 0), (b, 1)]));
        assert_eq!(g.distance(b), Ok(vec![(a, 1), (b, 0)]));
    }

    #[test]
    fn max_avatars() {
        let mut g = Graph::new();
        let a = g.add_node(Node::new(true));
        let b = g.add_node(Node::new(false));
        let c = g.add_node(Node::new(false));
        let d = g.add_node(Node::new(false));
        g.add_edge(a, b);
        g.add_edge(a, c);
        g.add_edge(b, d);
        g.add_edge(c, d);
        assert_eq!(g.max_avatars(a), (2, vec![d]));
    }

    #[test]
    fn avatar3() {
        //      a ----- b
        //      |       |  \
        //      |       |    e
        //      |       |  /
        //      c ----- d
        let mut g = Graph::new();
        let a = g.add_node(Node::new(true));
        let b = g.add_node(Node::new(false));
        let c = g.add_node(Node::new(false));
        let d = g.add_node(Node::new(false));
        let e = g.add_node(Node::new(false));
        g.add_edge(a, b);
        g.add_edge(a, c);
        g.add_edge(b, d);
        g.add_edge(c, d);
        g.add_edge(b, e);
        g.add_edge(d, e);
        assert_eq!(g.avatar_distance(a), vec![(0, 0), (1, 1), (2, 1), (3, 2), (4, 3)]);
    }

    #[test]
    fn contractible() {
        let mut g = Graph::new();
        let a = g.add_node(Node::new(true));
        let b = g.add_node(Node::new(false));
        let c = g.add_node(Node::new(false));
        g.add_edge(a, b);
        g.add_edge(b, c);
        assert_eq!(g.contractible(a), 1);
    }

    #[test]
    fn swap() {
        let mut g = Graph::new();
        let a = g.add_node(Node::new(true));
        let b = g.add_node(Node::new(false));
        let c = g.add_node(Node::new(false));
        g.add_edge(a, b);
        g.add_edge(a, c);
        assert_eq!(g.edges, vec![(0, 1), (0, 2)]);
        g.swap(a, b);
        assert_eq!(g.edges, vec![(0, 1), (1, 2)]);
    }

    #[test]
    fn avatar_graph() {
        let mut g = Graph::new();
        let a = g.add_node(Node::new(true));
        let b = g.add_node(Node::new(false));
        assert_eq!(g.is_avatar_graph(a), false);
        g.add_edge(a, b);
        assert_eq!(g.is_avatar_graph(a), true);
        assert_eq!(g.is_avatar_graph(b), true);
        let c = g.add_node(Node::new(false));
        assert_eq!(g.is_avatar_graph(a), false);
        g.add_edge(a, c);
        assert_eq!(g.is_avatar_graph(a), false);
        let d = g.add_node(Node::new(false));
        assert_eq!(g.is_avatar_graph(a), false);
        g.add_edge(c, d);
        assert_eq!(g.is_avatar_graph(a), false);
        g.add_edge(b, d);
        assert_eq!(g.is_avatar_graph(a), true);
    }

    #[test]
    fn corify() {
        let mut g = Graph::new();
        let a = g.add_node(Node::new(false));
        let b = g.add_node(Node::new(false));
        let c = g.add_node(Node::new(false));
        let d = g.add_node(Node::new(false));
        g.add_edge(a, b);
        g.add_edge(a, c);
        g.add_edge(b, d);
        g.add_edge(c, d);
        g.corify();
        assert_eq!(g.nodes[a].core, true);
        assert_eq!(g.nodes[b].core, true);
        assert_eq!(g.nodes[c].core, true);
        assert_eq!(g.nodes[d].core, true);
        assert_eq!(g.nodes[a].uniq, Some(d));
        assert_eq!(g.nodes[b].uniq, Some(c));
        assert_eq!(g.nodes[c].uniq, Some(b));
        assert_eq!(g.nodes[d].uniq, Some(a));

        let mut g = Graph::new();
        let a = g.add_node(Node::new(false));
        let b = g.add_node(Node::new(false));
        let c = g.add_node(Node::new(false));
        g.add_edge(a, b);
        g.add_edge(b, c);
        g.add_edge(c, a);
        g.corify();
        assert_eq!(g.cores(), 0);
    }

    #[test]
    fn corify_cube() {
        let mut g = Graph::new();
        let a000 = g.add_node(Node::new(false));
        let a100 = g.add_node(Node::new(false));
        let a010 = g.add_node(Node::new(false));
        let a001 = g.add_node(Node::new(false));
        let a011 = g.add_node(Node::new(false));
        let a101 = g.add_node(Node::new(false));
        let a110 = g.add_node(Node::new(false));
        let a111 = g.add_node(Node::new(false));
        g.add_edge(a000, a100);
        g.add_edge(a000, a010);
        g.add_edge(a000, a001);
        g.add_edge(a100, a110);
        g.add_edge(a100, a101);
        g.add_edge(a010, a110);
        g.add_edge(a010, a011);
        g.add_edge(a001, a101);
        g.add_edge(a001, a011);
        g.add_edge(a011, a111);
        g.add_edge(a101, a111);
        g.add_edge(a110, a111);
        g.corify();
        assert_eq!(g.cores(), 8);


        let mut g = Graph::new();
        let a000 = g.add_node(Node::new(false));
        let a110 = g.add_node(Node::new(false));
        let a101 = g.add_node(Node::new(false));
        let a100 = g.add_node(Node::new(false));
        let a111 = g.add_node(Node::new(false));
        let a010 = g.add_node(Node::new(false));
        let a001 = g.add_node(Node::new(false));
        let a011 = g.add_node(Node::new(false));
        g.add_edge(a010, a011);
        g.add_edge(a001, a011);
        g.add_edge(a000, a010);
        g.add_edge(a010, a110);
        g.add_edge(a101, a111);
        g.add_edge(a000, a001);
        g.add_edge(a011, a111);
        g.add_edge(a100, a110);
        g.add_edge(a100, a101);
        g.add_edge(a000, a100);
        g.add_edge(a001, a101);
        g.add_edge(a110, a111);
        g.corify();
        assert_eq!(g.cores(), 8);
    }

    #[test]
    fn corify_cube4() {
        let mut g = Graph {
            nodes: vec![Node::new(false); 16],
            edges: vec![
                (0, 3), (2, 3), (1, 2), (0, 1),
                (0, 4), (4, 7), (3, 7), (6, 7),
                (2, 6), (5, 6), (1, 5), (4, 5),
                (8, 15), (12, 15), (9, 12), (8, 9),
                (9, 11), (10, 11), (8, 10), (10, 14),
                (13, 14), (11, 13), (12, 13), (14, 15),
                (4, 15), (5, 12), (1, 9), (0, 8),
                (6, 13), (7, 14), (3, 10), (2, 11)
            ]
        };
        g.corify();
        assert_eq!(g.cores(), 16);
    }

    #[test]
    fn corify_5() {
        let mut g = Graph {
            nodes: vec![Node::new(false); 5],
            edges: vec![
                (0, 1), (1, 2),
                (2, 4), (3, 4),
                (0, 3), (2, 3)
            ]
        };
        g.corify();
        assert_eq!(g.cores(), 2);
    }

    #[test]
    fn corify_7() {
        let mut g = Graph {
            //     __ 6 __
            //   4 __   __  5
            //   | __ 2 __  |
            //   0 __   __  1
            //        3
            nodes: vec![Node::new(false); 7],
            edges: vec![
                (0, 3), (1, 3), (1, 2),
                (0, 2), (0, 4), (2, 4),
                (2, 5), (1, 5), (5, 6),
                (4, 6)
            ]
        };
        g.corify();
        assert_eq!(g.cores(), 2);
    }

    #[test]
    fn wagner() {
        //              1
        //         6    |    7
        //    2 ------- | ------- 3
        //         5    |    4
        //              0
        let mut g = Graph {
            nodes: vec![Node::new(false); 8],
            edges: vec![
                (0, 1), (2, 3), (5, 7), (4, 6),
                (0, 4), (0, 5), (2, 5), (2, 6),
                (1, 6), (1, 7), (3, 7), (3, 4)
            ]
        };
        g.corify();
        assert_eq!(g.cores(), 8);
    }

    #[test]
    fn corify_8() {
        //        0
        //     4 _  _ 6
        //  2   _ X _     3
        //     7      5
        //        1
        let mut g = Graph {
            nodes: vec![Node::new(false); 8],
            edges: vec![
                (0, 6), (3, 6), (3, 5),
                (1, 5), (1, 7), (2, 7),
                (2, 4), (0, 4), (4, 5),
                (6, 7)
            ]
        };
        g.corify();
        assert_eq!(g.cores(), 8);
    }

    #[test]
    fn corify_9() {
        //                   8
        //              /          \
        //          /                  \
        //        0------1-------2-------3
        //        |        \   /         |
        //        |         \/           |
        //        |         /\           |
        //        |       /    \         |
        //        4------5-------6-------7
        //          \                  /
        //              \         /
        //                   9
        let mut g = Graph {
            nodes: vec![Node { core: false, uniq: None }; 10],
            edges: vec![
                (0, 8), (3, 8), (0, 1), (1, 2),
                (2, 3), (0, 4), (1, 6), (2, 5),
                (3, 7), (4, 5), (5, 6), (6, 7),
                (4, 9), (7, 9)
            ]
        };
        g.corify();
        // assert_eq!(g.cores(), 4);
    }

    #[test]
    fn corify_10() {
        //  0 ------- 1
        //  |         |
        //  2         |
        // 4 3 ------ 5
        let mut g = Graph {
            nodes: vec![Node { core: false, uniq: None }; 6],
            edges: vec![
                (0, 1), (0, 2), (2, 4), (3, 4),
                (2, 3), (3, 5), (1, 5)
            ]
        };
        g.corify();
        // assert_eq!(g.cores(), 3);
    }
}
