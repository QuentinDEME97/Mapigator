# About this doc

This doc is here to provide a visual representation of test graphs.

# Base graph

This graph is used as a base example. Not relating to a specific test.

```mermaid
graph TD
    %% Node Styling
    0((0))
    1((1))
    2((2))
    3((3))
    4((4))
    5((5))
    6((6))

    %% Connections based on the Matrix
    0 <-->|1.0| 1
    0 <-->|12.0| 5
    0 <-->|7.0| 6

    1 <-->|2.0| 2

    2 <-->|3.0| 3
    2 -->|4.0| 4

    3 -->|2.0| 4
    3 <-->|5.0| 5

    5 <-->|10.0| 6

    %% Highlight Node 4 as a Sink
    style 4 stroke:#333,stroke-width:2px
```

# The dumbbell

**Usage:** This graph is used to test the Brandes algorithm. 2 denses communities are linked by a bridge.

**Tests asserts :**
  - Vertex `2 <-> 3` should have the highest Betweenness Score of the graph.
  - If we remove `2 <-> 3`, the graph is not anymore related.


```mermaid
graph TD
    0((0)) <--> 1((1))
    1 <--> 2((2))
    2 <--> 0
    2 <==>|Bridge| 3((3))
    3 <--> 4((4))
    4 <--> 5((5))
    5 <--> 3

    %% Style to highlight the bridge
    linkStyle 3 stroke:red,stroke-width:4px;
```

# Hotel California (The Trap)

**Usage:** This graph is used to test the Trajan algorithm. We can enter the red zone, but never come back to the green one.

**Tests asserts :**
  - Trajan should find 2 components {0, 1} and {2, 3, 4}.
  - Floyd-Warshall : `Distance (2, 0)` should be `INF`.
  - Floyd-Warshall : `Distance (0, 3)` should *not* be `INF`. 

```mermaid
graph TD
    subgraph Monde_Libre [Free area]
        0((0)) <--> 1((1))
    end

    1 -->|Oneway| 2((2))

    subgraph Prison [Sink]
        2 --> 3((3))
        3 --> 4((4))
        4 --> 2
    end

    %% Style
    style Monde_Libre stroke:#00b894
    style Prison stroke:#ff7675
    linkStyle 1 stroke:red,stroke-width:2px,stroke-dasharray: 5 5;
```

# Braess' Diamond

**Usage:** Test Dijkstra and shortcuts. A, B could be a nice shortcut.

**Tests asserts :**
  - If shortcut `B -> A` is very fast, everyone goes `S -> B -> A -> E`.
  - If `B -> A` has a low capacity, this link becomes a bottleneck.
  - In this paradox, add a link does not help but add more bottleneck.

```mermaid
graph LR
    S((Start)) -->|Long road| A((A))
    S -->|Short road| B((B))
    
    A -->|Short road| E((End))
    B -->|Long road| E
    
    B -.->|Tempting shortcut| A

    %% Style
    linkStyle 4 stroke:orange,stroke-width:3px,stroke-dasharray: 5 5;
```

# Grid vs Star

**Usage :** Testing Tortuosity and Resilience.

## Star (Centralized and Fragile)

- HUB has a maximum centrality.
- Removing the HUB destoys everything.

```mermaid
graph TD
    Hub((HUB))
    
    P1((P1)) <--> Hub
    P2((P2)) <--> Hub
    P3((P3)) <--> Hub
    P4((P4)) <--> Hub
    
    %% Pas de liens entre P1, P2, etc.
    style Hub fill:#f9ca24,stroke:#333,stroke-width:2px
```
## The grid (Decentralized / Resilient)

- Removing a node does not affect accessibility.

```mermaid
graph TD
    A1((0)) <--> A2((1))
    A1 <--> B1((3))
    
    A2 <--> B2((2))
    B1 <--> B2
    
    %% Diagonales optionnelles pour la robustesse ++
```