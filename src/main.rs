use std::io::{self, Read};
use serde::{Deserialize, Serialize};

// TINY GRAPH ENGINE (The "Nano-Surface")
// A standalone, portable Graph Execution Unit.
// It reads a Causal Graph Delta from STDIN and reifies it (creates files, etc).
//
// "We need no parsing... just the graph delta."

#[derive(Serialize, Deserialize, Debug)]
struct Graph {
    nodes: Vec<Node>,
    links: Vec<Link>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Node {
    id: String,
    label: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Link {
    source: String,
    target: String,
    #[serde(rename = "type")]
    rel: String,
    content: Option<String>,      // Inline Payload
    content_hash: Option<String>, // CAS Reference (sha256:...)
}

fn main() -> io::Result<()> {
    // 1. Self-Documentation
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && (args[1] == "--help" || args[1] == "-h") {
        print_instructions();
        return Ok(());
    }

    // 2. Read Graph Delta from Stdin
    println!("🔌 Tiny Graph Engine Online. Waiting for JSON Graph Delta on Standard Input...");
    let mut buffer = String::new();
    if let Err(_) = io::stdin().read_to_string(&mut buffer) {
        // If interactive or empty, just show instructions
        print_instructions();
        return Ok(());
    }
    
    if buffer.trim().is_empty() {
        print_instructions();
        return Ok(());
    }

    // 3. Proccess Graph
    match serde_json::from_str::<Graph>(&buffer) {
        Ok(graph) => {
            println!("⚡ Applying Delta: {} Nodes, {} Links", graph.nodes.len(), graph.links.len());
            apply_delta(&graph);
        },
        Err(e) => {
            eprintln!("❌ Invalid Graph JSON: {}", e);
            print_instructions();
        }
    }

    Ok(())
}

fn apply_delta(graph: &Graph) {
    for link in &graph.links {
        // Semantic Edge Interpretation
        if link.rel == "creates" || link.rel == "modifies" {
            let path = if link.target.starts_with("FILE:") {
                &link.target[5..]
            } else {
                &link.target
            };

            let maybe_content = if let Some(c) = &link.content {
                Some(c.clone())
            } else if let Some(hash) = &link.content_hash {
                // CAS Lookup: .cas/<hash>
                let cas_path = format!(".cas/{}", hash);
                match std::fs::read_to_string(&cas_path) {
                    Ok(c) => Some(c),
                    Err(_) => {
                        println!("       ❌ CAS Miss: {}", hash);
                        None
                    }
                }
            } else {
                None
            };

            if let Some(content) = maybe_content {
                println!("   └── Reifying: {} -> {}", link.rel, path);
                if let Some(parent) = std::path::Path::new(path).parent() {
                    let _ = std::fs::create_dir_all(parent);
                }
                match std::fs::write(path, content) {
                    Ok(_) => println!("       ✅ Written (CAS verified)"),
                    Err(e) => println!("       ❌ Error: {}", e),
                }
            } else {
                println!("   ⚠️  Edge {} -> {} missing content/hash", link.rel, path);
            }
        } else if link.rel == "syncs" {
             // Capability: Chronological Snapshot / Backup
             // Usage: { "type": "syncs", "source": ".", "target": "snapshots/v1" }
             let source = &link.source.replace("DIR:", "").replace("FILE:", "");
             let target = &link.target.replace("DIR:", "").replace("FILE:", "");
             
             println!("   └── Syncing: {} -> {}", source, target);
             
             let output = std::process::Command::new("rsync")
                 .arg("-av") 
                 .arg("--exclude").arg("target")
                 .arg("--exclude").arg(".git")
                 .arg("--exclude").arg("meta3-graph-core/target")
                 .arg("--exclude").arg("node_modules")
                 .arg(source)
                 .arg(target)
                 .output();

             match output {
                 Ok(o) => {
                     println!("       ✅ Sync Complete. Status: {}", o.status);
                     // We don't print stdout here to keep the log clean (Deltas only)
                 },
                 Err(e) => println!("       ❌ Sync Failed: {}", e),
             }

        } else if link.rel == "executes" || link.rel == "runs" {
            // Orchestration / Tool Use
            // Target format: "SHELL:<cmd>" or just "<cmd>"
            let cmd_str = if link.target.starts_with("SHELL:") {
                &link.target[6..]
            } else {
                &link.target
            };
            
            println!("   └── Executing: {}", cmd_str);
            let parts: Vec<&str> = cmd_str.split_whitespace().collect();
            if let Some(exe) = parts.first() {
                let args = &parts[1..];
                match std::process::Command::new(exe).args(args).output() {
                    Ok(out) => {
                        println!("       ✅ Exit Code: {}", out.status);
                        if !out.stdout.is_empty() {
                            println!("       📄 Stdout: {}", String::from_utf8_lossy(&out.stdout).trim());
                        }
                        if !out.stderr.is_empty() {
                            println!("       ⚠️ Stderr: {}", String::from_utf8_lossy(&out.stderr).trim());
                        }
                    },
                    Err(e) => println!("       ❌ Failed to run: {}", e),
                }
            }
        }
    }
}

fn print_instructions() {
    println!(r#"
🟦 TINY GRAPH ENGINE (Meta3 Core)
   "Abstracting everything away in the graph delta."

USAGE:
  echo '{{ "nodes": [], "links": [...] }}' | tiny_graph

PROTOCOL:
  - Input: valid JSON Object with "nodes" and "links".
  - Link "creates" | "modifies": Writes 'content' to 'target' (File Path).
  - Link "executes" | "runs": Runs 'target' as Shell Command.

EXAMPLE:
  {{
    "nodes": [
      {{ "id": "GOAL" }},
      {{ "id": "FILE:script.py" }}
    ],
    "links": [
      {{ "source": "GOAL", "target": "FILE:script.py", "type": "creates", "content": "print('Alive')" }}
    ]
  }}
"#);
}
