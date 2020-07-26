# Avatar Graph

A library for Avatar Graphs.

![avatar5-01.png](https://raw.githubusercontent.com/advancedresearch/avatar_graph/master/images/avatar5-01.png)
![avatar8-02.png](https://raw.githubusercontent.com/advancedresearch/avatar_graph/master/images/avatar8-02.png)
![avatar3-01.png](https://raw.githubusercontent.com/advancedresearch/avatar_graph/master/images/avatar3-01.png)

*An Avatar Graph is a special kind of graph that is useful for mathematical programming.*

An Avatar Graph is an sub-type of graphs
which is used as a theorem proving technique/tool in [Path Semantics](https://github.com/advancedresearch/path_semantics).

For more information, see the paper [Avatar Graphs](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/avatar-graphs.pdf).

### Introduction to Avatar Graphs

![avatar16-01.png](https://raw.githubusercontent.com/advancedresearch/avatar_graph/master/images/avatar16-01.png)

*A hypercube of 4 dimensions. Avatar Graphs can be used to prove isomorphism of graphs to hypercubes.*

A more practical example for programmers: [The New Type Idiom in Rust](https://doc.rust-lang.org/rust-by-example/generics/new_types.html).

- Why does it technique work?
- How can one prove that such techniques work in general in mathematics?
- How can such techniques be generalized?

It turns out that there is a mathematical pattern behind such generalizations.

At abstract level, this pattern takes the form of an Avatar Graph.

Avatar Graphs are first and foremost a tool for insight and inspiration.
They are usually not needed to be represented explicitly.
This means Avatar Graphs can tell something about mathematical theories
that is not clear, seen from within the theory.

### Diagrams of Avatar Graphs

The original mathematical model is called the "core self" or just "core" in an Avatar Graph.
Avatar Graphs are used to study symmetries of core selfs from avatar extensions.

- black node: Candidate for core self
- white node: N-avatar where where the smallest possible `N` is greater than zero
- black edge: Directed edge depending on choice of core self candidate
- grey/dashed edge: A unique edge from a core self candidate to highest avatar

![avatar7-01.png](https://raw.githubusercontent.com/advancedresearch/avatar_graph/master/images/avatar7-01.png)

### Unique Highest Avatar

Each candidate core has an associated unique highest avatar,
due to removal of all irrelevant structure from the graph.

This is because adding new n-avatars at same level is trivial,
so it is only interesting to study graphs whith a unique highest avatar.

The choice of highest avatar might depend on order of nodes.
This limitation is mostly irrelevant for most interesting graphs.
However, it greatly simplifies the calculation of avatar distances.

### Avatar Distances

The `n` in n-avatar is also a kind of distance in the graph, called "avatar distance".

An avatar distance counts the number of paths to the core self.

It is constructed by first finding the shortest distance for all nodes,
and then "relaxed" by summing avatar distances of children.
The choice of children might depend on the order of nodes.

The exact value of an avatar distance does not matter,
except as a mean to find the highest avatar relative to a core candidate.

Think of it of as a somewhat complex puzzle that needs to be solved,
and the avatar distances is just a technique for solving the puzzle.

Avatar distances have different semantics than the shortest distance.
Two nodes with equal shortest distance can have different avatar distances.

### Avatar Connectivity

Avatars at the same level are not allowed to communicate.

A 1-avatar can only pass information to the core self.

A 2-avatar can only pass information to two 1-avatars.

A 3-avatar can pass information to 3x1-avatars or 2+1-avatars.

An n-avatar can pass information to any partition of lower avatars.

### Non-Contractability

When an avatar has only one child, it is contractible,
which is not permitted relative to a core candidate.

The core self is not counted as a child for contractability.
This means 1-avatars have zero children, therefore not one child.
One can also think about it as 1-avatars being an exception from the rule.

### Universal Reachability

All nodes in the graph must be reachable when descending along the gradient
from highest avatar to the core candidate.

With other words, there exists some path from the highest avatar to any node,
while getting closer to the core all the way.

Another way of thinking about this property, is that for a well-chosen path
from the highest avatar to the core, one can visit any node.

This property is beneficial in systems where you want to have choices,
but you also want to make continuous progress.

### Avatar Graphs and Hypercubes

Conjecture: Any Avatar Graphs containing only core candidates is
isomorphic to some hypercube.

If this conjecture is correct, then the number of nodes required
to construct a "filled" Avatar Graph is:

`2^n` where `n = 0, 1, 2, ...`.
