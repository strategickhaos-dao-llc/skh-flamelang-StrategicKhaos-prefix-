//! # AetherViz - Self-Referential Repo Visualization & Sonification
//!
//! Core module for visualizing and sonifying the repository structure.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use super::aetherlingua::AetherLingua;

/// Represents a node in the repository tree
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoNode {
    pub path: PathBuf,
    pub name: String,
    pub is_dir: bool,
    pub depth: usize,
    pub file_type: Option<String>,
    pub size: u64,
    pub children: Vec<RepoNode>,
}

/// Represents the full repository tree structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoTree {
    pub root: RepoNode,
    pub node_count: usize,
    pub max_depth: usize,
    pub file_types: HashMap<String, usize>,
}

impl RepoTree {
    /// Get all files in the tree (flattened)
    pub fn files(&self) -> Vec<&RepoNode> {
        let mut result = Vec::new();
        self.collect_files(&self.root, &mut result);
        result
    }

    fn collect_files<'a>(&self, node: &'a RepoNode, result: &mut Vec<&'a RepoNode>) {
        if !node.is_dir {
            result.push(node);
        }
        for child in &node.children {
            self.collect_files(child, result);
        }
    }

    /// Check if the tree contains files of a specific type
    pub fn contains_type(&self, ext: &str) -> bool {
        self.file_types.contains_key(ext)
    }
}

/// Output structure containing all visualizations
#[derive(Debug, Serialize, Deserialize)]
pub struct Visualization {
    pub svg_graph: String,
    pub audio_description: String,
    pub sonic_hash: String,
    pub dna_proof: String,
    pub on_chain_payload: String,
    pub obsidian_md: String,
}

/// Main AetherViz structure
pub struct AetherViz {
    lingua: AetherLingua,
}

impl AetherViz {
    /// Create a new AetherViz instance
    pub fn new() -> Self {
        Self {
            lingua: AetherLingua::new(),
        }
    }

    /// Visualize the entire repository
    pub fn visualize_repo(&self, repo_path: &Path) -> Result<Visualization, std::io::Error> {
        let tree = self.build_tree(repo_path)?;
        let svg = self.render_graph_svg(&tree);
        let audio = self.lingua.render_tree(&tree);
        
        // Compute hash
        let mut hasher = Sha256::new();
        hasher.update(&svg);
        hasher.update(&audio);
        let hash = hasher.finalize();
        let sonic_hash = hex::encode(hash);
        
        let dna = self.dna_encode_tree(&tree);
        let payload = self.prepare_payload(&sonic_hash, &tree);
        let obsidian_md = self.generate_obsidian_md(&tree);

        Ok(Visualization {
            svg_graph: svg,
            audio_description: audio,
            sonic_hash,
            dna_proof: dna,
            on_chain_payload: payload,
            obsidian_md,
        })
    }

    /// Build the repository tree structure
    pub fn build_tree(&self, path: &Path) -> Result<RepoTree, std::io::Error> {
        let mut node_count = 0;
        let mut max_depth = 0;
        let mut file_types: HashMap<String, usize> = HashMap::new();

        let root = self.build_node(path, 0, &mut node_count, &mut max_depth, &mut file_types)?;

        Ok(RepoTree {
            root,
            node_count,
            max_depth,
            file_types,
        })
    }

    fn build_node(
        &self,
        path: &Path,
        depth: usize,
        node_count: &mut usize,
        max_depth: &mut usize,
        file_types: &mut HashMap<String, usize>,
    ) -> Result<RepoNode, std::io::Error> {
        *node_count += 1;
        *max_depth = (*max_depth).max(depth);

        let metadata = std::fs::metadata(path)?;
        let is_dir = metadata.is_dir();
        let size = metadata.len();
        let name = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        let file_type = if !is_dir {
            path.extension()
                .map(|ext| ext.to_string_lossy().to_string())
                .map(|ext| {
                    *file_types.entry(ext.clone()).or_insert(0) += 1;
                    ext
                })
        } else {
            None
        };

        let mut children = Vec::new();
        if is_dir {
            if let Ok(entries) = std::fs::read_dir(path) {
                for entry in entries.flatten() {
                    let entry_path = entry.path();
                    let entry_name = entry_path
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string();

                    // Skip hidden files, .git, target, node_modules, etc.
                    if entry_name.starts_with('.')
                        || entry_name == "target"
                        || entry_name == "node_modules"
                        || entry_name == "dist"
                    {
                        continue;
                    }

                    if let Ok(child) =
                        self.build_node(&entry_path, depth + 1, node_count, max_depth, file_types)
                    {
                        children.push(child);
                    }
                }
            }
        }

        Ok(RepoNode {
            path: path.to_path_buf(),
            name,
            is_dir,
            depth,
            file_type,
            size,
            children,
        })
    }

    /// Render the repository as an SVG graph
    pub fn render_graph_svg(&self, tree: &RepoTree) -> String {
        let mut svg = String::new();
        svg.push_str(r#"<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" width="1200" height="800" viewBox="0 0 1200 800">
  <defs>
    <style>
      .node { stroke: #333; stroke-width: 2; }
      .node-mojo { fill: #ff6b35; }
      .node-rs { fill: #ce422b; }
      .node-md { fill: #6ab7ff; }
      .node-toml { fill: #9c4221; }
      .node-dir { fill: #ffcc00; }
      .node-other { fill: #cccccc; }
      .edge { stroke: #999; stroke-width: 1; opacity: 0.6; }
      .text { font-family: monospace; font-size: 10px; fill: #333; }
    </style>
  </defs>
  <title>AetherViz Repository Brain - Self-Referential Graph</title>
"#);

        // Generate nodes and edges
        let mut y_offset = 50;
        self.render_node_svg(&tree.root, 100, &mut y_offset, &mut svg);

        svg.push_str("</svg>");
        svg
    }

    fn render_node_svg(
        &self,
        node: &RepoNode,
        x: i32,
        y_offset: &mut i32,
        svg: &mut String,
    ) {
        let y = *y_offset;
        *y_offset += 30;

        let color_class = if node.is_dir {
            "node-dir"
        } else {
            match node.file_type.as_deref() {
                Some("mojo") => "node-mojo",
                Some("rs") => "node-rs",
                Some("md") => "node-md",
                Some("toml") => "node-toml",
                _ => "node-other",
            }
        };

        let escaped_name = node.name
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&apos;");

        svg.push_str(&format!(
            r#"  <circle cx="{}" cy="{}" r="8" class="node {}" />
  <text x="{}" y="{}" class="text">{}</text>
"#,
            x,
            y,
            color_class,
            x + 15,
            y + 5,
            escaped_name
        ));

        if node.is_dir {
            for child in &node.children {
                let child_x = x + 20 * (node.depth as i32 + 1);
                let child_y = *y_offset;
                svg.push_str(&format!(
                    r#"  <line x1="{}" y1="{}" x2="{}" y2="{}" class="edge" />
"#,
                    x, y, child_x, child_y
                ));
                self.render_node_svg(child, child_x, y_offset, svg);
            }
        }
    }

    /// Generate Obsidian-compatible markdown
    pub fn generate_obsidian_md(&self, tree: &RepoTree) -> String {
        let mut md = String::from("# AetherForge Repository Brain\n\n");
        md.push_str("```tree\n");
        self.tree_string(&tree.root, 0, &mut md);
        md.push_str("```\n\n## Files\n\n");

        for file in tree.files() {
            let path_str = file.path.to_string_lossy();
            md.push_str(&format!("- [[{}]]\n", path_str));
        }

        md.push_str("\n## Statistics\n\n");
        md.push_str(&format!("- **Total Nodes**: {}\n", tree.node_count));
        md.push_str(&format!("- **Max Depth**: {}\n", tree.max_depth));
        md.push_str("\n### File Types\n\n");
        for (ext, count) in &tree.file_types {
            md.push_str(&format!("- `.{}`: {} files\n", ext, count));
        }

        md
    }

    fn tree_string(&self, node: &RepoNode, indent: usize, output: &mut String) {
        let prefix = "  ".repeat(indent);
        let icon = if node.is_dir { "📁" } else { "📄" };
        output.push_str(&format!("{}{} {}\n", prefix, icon, node.name));

        for child in &node.children {
            self.tree_string(child, indent + 1, output);
        }
    }

    /// DNA-encode the tree structure
    pub fn dna_encode_tree(&self, tree: &RepoTree) -> String {
        // Convert tree structure to DNA sequence (A, T, G, C)
        let mut hasher = Sha256::new();
        hasher.update(serde_json::to_string(tree).unwrap_or_default().as_bytes());
        let hash = hasher.finalize();
        
        // Map hash bytes to DNA bases
        let mut dna = String::new();
        for byte in hash.iter() {
            let bases = match byte % 4 {
                0 => "AT",
                1 => "GC",
                2 => "TA",
                _ => "CG",
            };
            dna.push_str(bases);
        }
        dna
    }

    /// Prepare on-chain payload
    pub fn prepare_payload(&self, hash: &str, tree: &RepoTree) -> String {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_else(|_| std::time::Duration::from_secs(0))
            .as_secs();

        let mut triggers = Vec::new();
        if tree.contains_type("mojo") {
            triggers.push("flame_resonance");
        }
        if tree.contains_type("rs") {
            triggers.push("rust_oxide");
        }

        let payload = serde_json::json!({
            "type": "aetherviz_repo_brain",
            "repo_hash": hash,
            "node_count": tree.node_count,
            "depth": tree.max_depth,
            "timestamp": timestamp,
            "triggers": triggers,
            "file_types": tree.file_types,
        });

        serde_json::to_string_pretty(&payload).unwrap_or_default()
    }
}

impl Default for AetherViz {
    fn default() -> Self {
        Self::new()
    }
}
