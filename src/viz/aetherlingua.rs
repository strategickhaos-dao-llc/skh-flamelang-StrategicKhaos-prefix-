//! # AetherLingua - Sonification Engine
//!
//! Converts repository structure into sonic representations.
//! 
//! Sonification Rules:
//! - Directory depth → carrier frequency (deeper = lower pitch)
//! - File type → timbre (mojo = flame crackle, rs = metallic ring, md = soft pad)
//! - File count → rhythm density
//! - Special patterns for treasury files and quantum modules

use super::aetherviz::{RepoNode, RepoTree};

/// AetherLingua sonification engine
pub struct AetherLingua;

impl AetherLingua {
    /// Create a new AetherLingua instance
    pub fn new() -> Self {
        Self
    }

    /// Render the repository tree as audio description
    pub fn render_tree(&self, tree: &RepoTree) -> String {
        let mut description = String::from("🎵 AetherLingua Sonification\n\n");
        
        description.push_str("## Sonic Structure\n\n");
        description.push_str(&format!(
            "**Base Frequency**: {} Hz (root depth)\n",
            self.depth_to_frequency(0)
        ));
        description.push_str(&format!(
            "**Max Frequency**: {} Hz (depth {})\n",
            self.depth_to_frequency(tree.max_depth),
            tree.max_depth
        ));
        description.push_str(&format!("**Node Density**: {} nodes/measure\n\n", tree.node_count));

        description.push_str("## Timbral Map\n\n");
        for (ext, count) in &tree.file_types {
            let timbre = self.file_type_to_timbre(ext);
            description.push_str(&format!("- `.{}` ({} files): {}\n", ext, count, timbre));
        }

        description.push_str("\n## Sonic Journey\n\n");
        self.describe_node_sonification(&tree.root, &mut description);

        // Special patterns
        if tree.contains_type("toml") && tree.file_types.get("toml").unwrap_or(&0) > &0 {
            description.push_str("\n### 🎼 7% Charity Motif Detected\n");
            description.push_str("Treasury configuration files trigger the sovereignty harmonic.\n");
        }

        if tree.contains_type("quantum") || tree.contains_type("q") {
            description.push_str("\n### ⚛️ Node 137 Quantum Burst\n");
            description.push_str("Quantum modules activate fine-structure constant resonance (137 Hz).\n");
        }

        description
    }

    fn describe_node_sonification(&self, node: &RepoNode, output: &mut String) {
        let freq = self.depth_to_frequency(node.depth);
        let timbre = if node.is_dir {
            "ambient pad (directory)"
        } else {
            self.file_type_to_timbre(node.file_type.as_deref().unwrap_or("unknown"))
        };

        output.push_str(&format!(
            "- **{}** (depth {}): {} Hz, {}\n",
            node.name, node.depth, freq, timbre
        ));

        if node.is_dir && !node.children.is_empty() {
            let indent = "  ".repeat(node.depth + 1);
            for child in &node.children {
                output.push_str(&indent);
                self.describe_node_sonification(child, output);
            }
        }
    }

    /// Map directory depth to carrier frequency
    /// Deeper = lower pitch (bass foundation)
    fn depth_to_frequency(&self, depth: usize) -> f32 {
        let base_freq = 440.0; // A4
        let octave_factor = 2.0_f32.powf(-(depth as f32) / 3.0);
        base_freq * octave_factor
    }

    /// Map file type to timbre description
    fn file_type_to_timbre(&self, ext: &str) -> &str {
        match ext {
            "mojo" => "🔥 flame crackle overtone (warm sawtooth + noise burst)",
            "rs" => "⚙️ metallic ring (bright square wave + metallic resonance)",
            "md" => "📝 soft pad (gentle sine wave with reverb)",
            "toml" => "🎹 piano stab (percussive attack, harmonic decay)",
            "yml" | "yaml" => "🌊 water flow (filtered white noise)",
            "json" => "🔔 bell tone (pure sine with envelope)",
            "html" | "htm" => "🌐 digital chirp (FM synthesis)",
            "css" => "🎨 color wash (harmonic series)",
            "js" | "ts" => "⚡ electric pulse (pulse wave + filter sweep)",
            "py" => "🐍 serpentine glide (portamento between notes)",
            "go" => "🏃 rhythmic stomp (kick drum pattern)",
            "java" => "☕ brewing bubbles (granular synthesis)",
            "c" | "cpp" | "h" | "hpp" => "🔩 industrial clang (metallic percussion)",
            _ => "🎵 soft synth (basic waveform)",
        }
    }

    /// Generate rhythm based on file count
    pub fn generate_rhythm(&self, file_count: usize) -> String {
        let density = (file_count as f32 / 10.0).min(1.0);
        format!(
            "Rhythm density: {:.1}% ({}bpm equivalent)",
            density * 100.0,
            (60.0 + density * 120.0) as u32
        )
    }
}

impl Default for AetherLingua {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_depth_to_frequency() {
        let lingua = AetherLingua::new();
        let root_freq = lingua.depth_to_frequency(0);
        assert_eq!(root_freq, 440.0);
        
        let deeper_freq = lingua.depth_to_frequency(3);
        assert!(deeper_freq < root_freq);
    }

    #[test]
    fn test_file_type_to_timbre() {
        let lingua = AetherLingua::new();
        assert!(lingua.file_type_to_timbre("mojo").contains("flame"));
        assert!(lingua.file_type_to_timbre("rs").contains("metallic"));
        assert!(lingua.file_type_to_timbre("md").contains("soft"));
    }

    #[test]
    fn test_generate_rhythm() {
        let lingua = AetherLingua::new();
        let rhythm = lingua.generate_rhythm(50);
        assert!(rhythm.contains("bpm"));
    }
}
