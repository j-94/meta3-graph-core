use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let out_dir = if let Some(idx) = args.iter().position(|r| r == "--out") {
        args.get(idx + 1).cloned().unwrap_or_else(|| "runs/toolhub".to_string())
    } else {
        "runs/toolhub".to_string()
    };

    fs::create_dir_all(&out_dir)?;

    // 1. Build Catalog
    let catalog = serde_json::json!({
        "generated_at": "2025-12-16",
        "engine": { "name": "Meta3 Graph Core", "version": "0.1.0" },
        "tools": [
            {
                "id": "meta3-graph-core",
                "kind": "graph-kernel",
                "where": { "binary": "src/bin/tiny_graph.rs" },
                "notes": ["The Unikernel. Accepts JSON Graph Deltas via Stdin."]
            },
            {
                "id": "ruliad.kernel",
                "kind": "ruliad",
                "where": { "endpoint": "/run" },
                "notes": ["Generates computational universes via Wolfram Rules."]
            },
            {
                "id": "genesis.loop",
                "kind": "meta",
                "where": { "endpoint": "/run" },
                "notes": ["The Self-Improvement Loop."]
            },
            {
                "id": "toolhub",
                "kind": "cli",
                "where": { "binary": "src/bin/toolhub.rs" },
                "notes": ["The Capability Catalog."]
            }
        ],
        "features": [
            {
                "id": "generalized-graph-system",
                "summary": "All system behavior becomes graphs (receipts, traces, ruliad, task graphs).",
                "entrypoints": [
                    "mini graph: meta3-graph-core",
                    "ledger: knowledge.graph.jsonl"
                ]
            }
        ]
    });

    // 2. Write JSON
    let json_path = Path::new(&out_dir).join("tools.json");
    let mut file = File::create(&json_path)?;
    file.write_all(serde_json::to_string_pretty(&catalog)?.as_bytes())?;
    println!("✅ Wrote catalog to {:?}", json_path);

    // 3. Write HTML
    let html_path = Path::new(&out_dir).join("index.html");
    let mut file = File::create(&html_path)?;
    let html = format!(r#"<!DOCTYPE html>
<html>
<head><title>Meta3 ToolHub</title>
<style>
body {{ font-family: sans-serif; background: #111; color: #eee; padding: 20px; }}
.tool {{ background: #222; padding: 10px; margin: 10px 0; border-left: 4px solid #00ff00; }}
pre {{ background: #000; padding: 15px; border-radius: 5px; overflow-x: auto; }}
</style>
</head>
<body>
<h1>🔵 Meta3 ToolHub</h1>
<p>The Unified Capability Catalog</p>
<div class="tool">
<h3>System Manifest</h3>
<pre>{}</pre>
</div>
</body>
</html>"#, serde_json::to_string_pretty(&catalog)?);
    file.write_all(html.as_bytes())?;
    println!("✅ Wrote viewer to {:?}", html_path);

    Ok(())
}