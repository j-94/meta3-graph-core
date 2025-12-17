---
title: Meta3 Graph Core
theme: jekyll-theme-minimal
---

# The Meta3 Graph Core

**The Isomorphic Engine for AGI.**

## The Problem: The Anthropomorphic Valley
We have been trying to make AI act like a human programmer (CodeAct). It types scripts, runs them, parses errors. This is slow, fragile, and high-entropy.

## The Solution: Isomorphic Agency
Instead of "Simulating a User", we **Access the Physics**.
The Meta3 Graph Core is a 150-line Rust binary that turns **Graph Deltas** (Intent) directly into **Reality** (Files/Processes).

### The Evolution Graph
![Evolution](https://mermaid.ink/img/pako:eNp1ksFu2zAMhl8F6NQC_QAv3Q4Ddtih2_bSwyDIcWIrkSJBp2mG4HePsp2mzYADK_TjT_xTyA8lWw0Wymf1W8F7I1sN_0qV0-fLy-u3t8v5_e3i5fX94vX1cnE5n_7K9tJp-FvIP0_P768v387v396-fX37-fXj4vPj4vPz-eI-n99fz-fzxft8Pp_f5_P5fH6fz-fz-X0-n8_n9_l8Pp_f5_P5) 
*(See diagram in repo for details)*

## Recursive Language Models (RLM)
Moving beyond CodeAct requires **Recursion**.

*   **Standard CodeAct**: `Context -> LLM -> Action -> Report`.
*   **RLM (Meta3)**: `Context + Graph -> LLM -> Graph Delta (Modify Self) -> Action`.

Because the **Graph is the Memory**, the Agent can recursively refine its own topology. It allows "Reverse Recursion":
> The output of the agent (The Graph Delta) becomes the input for the next agent (The Graph Tape).

This closes the loop, allowing the system to self-improve infinitely (Autopoiesis).

## Usage
```bash
echo '{ "nodes": [{"id":"GOAL"}, {"id":"FILE:hello.txt"}], "links": [{"source":"GOAL", "target":"FILE:hello.txt", "type":"creates", "content":"Hello World"}] }' | ./meta3-graph-core
```

[View on GitHub](https://github.com/j-94/meta3-graph-core)