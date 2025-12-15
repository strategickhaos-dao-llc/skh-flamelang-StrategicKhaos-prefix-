use clap::Parser;
use reqwest::blocking::Client;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use url::Url;

/// Sovereign Page Cloning & Synthesis Engine - INV-079
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// URL to clone
    #[arg(short, long)]
    url: String,

    /// Output directory
    #[arg(short, long, default_value = "output")]
    output: PathBuf,

    /// Generate cryptographic proof
    #[arg(short, long)]
    proof: bool,

    /// Inline all assets (images, CSS, JS)
    #[arg(short, long)]
    inline: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct CloneProof {
    url: String,
    timestamp: String,
    content_hash: String,
    proof_type: String,
    assets: Vec<AssetProof>,
}

#[derive(Serialize, Deserialize, Debug)]
struct AssetProof {
    url: String,
    hash: String,
    mime_type: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    println!("🔮 VesselMirror - Sovereign Page Cloning Engine");
    println!("📍 Target: {}", args.url);

    // Create output directory
    fs::create_dir_all(&args.output)?;

    // Fetch the page
    let client = Client::builder()
        .user_agent("VesselMirror/0.1.0")
        .build()?;
    
    let response = client.get(&args.url).send()?;
    let base_url = Url::parse(&args.url)?;
    let html_content = response.text()?;

    // Parse HTML
    let document = Html::parse_document(&html_content);
    
    // Calculate content hash
    let mut hasher = Sha256::new();
    hasher.update(html_content.as_bytes());
    let content_hash = hex::encode(hasher.finalize());

    println!("✅ Page fetched successfully");
    println!("📊 Content hash: {}", content_hash);

    // Process assets if inline mode is enabled
    let mut assets = Vec::new();
    let mut processed_html = html_content.clone();

    if args.inline {
        println!("🔄 Inlining assets...");
        assets = inline_assets(&client, &document, &base_url, &mut processed_html)?;
        println!("✅ Inlined {} assets", assets.len());
    }

    // Save the cloned page
    let output_file = args.output.join("index.html");
    fs::write(&output_file, &processed_html)?;
    println!("💾 Saved to: {}", output_file.display());

    // Generate proof if requested
    if args.proof {
        let proof = CloneProof {
            url: args.url.clone(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            content_hash,
            proof_type: "vesselmirror_sovereign".to_string(),
            assets,
        };

        let proof_file = args.output.join("proof.json");
        fs::write(&proof_file, serde_json::to_string_pretty(&proof)?)?;
        println!("🔐 Cryptographic proof saved to: {}", proof_file.display());
    }

    println!("✨ Clone complete!");
    Ok(())
}

fn inline_assets(
    client: &Client,
    document: &Html,
    base_url: &Url,
    html: &mut String,
) -> Result<Vec<AssetProof>, Box<dyn std::error::Error>> {
    let mut assets = Vec::new();
    let mut replacements: HashMap<String, String> = HashMap::new();

    // Process images
    let img_selector = Selector::parse("img").unwrap();
    for element in document.select(&img_selector) {
        if let Some(src) = element.value().attr("src") {
            if let Ok(asset_url) = base_url.join(src) {
                if let Ok(data) = fetch_and_encode_asset(client, &asset_url) {
                    replacements.insert(src.to_string(), data.data_url.clone());
                    assets.push(data.proof);
                }
            }
        }
    }

    // Process CSS links
    let link_selector = Selector::parse("link[rel=\"stylesheet\"]").unwrap();
    for element in document.select(&link_selector) {
        if let Some(href) = element.value().attr("href") {
            if let Ok(asset_url) = base_url.join(href) {
                if let Ok(data) = fetch_and_encode_asset(client, &asset_url) {
                    replacements.insert(href.to_string(), data.data_url.clone());
                    assets.push(data.proof);
                }
            }
        }
    }

    // Apply replacements
    for (original, replacement) in replacements {
        *html = html.replace(&original, &replacement);
    }

    Ok(assets)
}

struct AssetData {
    data_url: String,
    proof: AssetProof,
}

fn fetch_and_encode_asset(
    client: &Client,
    url: &Url,
) -> Result<AssetData, Box<dyn std::error::Error>> {
    let response = client.get(url.as_str()).send()?;
    let content = response.bytes()?;
    
    // Calculate hash
    let mut hasher = Sha256::new();
    hasher.update(&content);
    let hash = hex::encode(hasher.finalize());

    // Guess MIME type
    let mime_type = mime_guess::from_path(url.path())
        .first_or_octet_stream()
        .to_string();

    // Create data URL
    let encoded = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &content);
    let data_url = format!("data:{};base64,{}", mime_type, encoded);

    Ok(AssetData {
        data_url,
        proof: AssetProof {
            url: url.to_string(),
            hash,
            mime_type,
        },
    })
}
