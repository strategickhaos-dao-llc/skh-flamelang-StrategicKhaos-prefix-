//! FlameVault CLI — Honeypot + Encrypted Secrets Engine
//! INV-080: Strategickhaos DAO LLC

use clap::{Parser, Subcommand};
use flamelang::flamevault::{FlameVault, SecretResult};

#[derive(Parser)]
#[command(name = "flamevault")]
#[command(author = "Strategickhaos DAO LLC <security@strategickhaos.ai>")]
#[command(version = "0.1.0")]
#[command(about = "🔥 Honeypot + Encrypted Secrets Engine", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize FlameVault and show status
    Init,
    
    /// Store an encrypted secret (machine-bound)
    Set {
        /// Secret name
        name: String,
        /// Secret value
        value: String,
    },
    
    /// Retrieve a secret (decrypts if real, returns bait if honeypot)
    Get {
        /// Secret name
        name: String,
        
        /// Output raw value only (for scripts)
        #[arg(short, long)]
        raw: bool,
    },
    
    /// Deploy a honeypot (bait) key
    Honeypot {
        /// Key name
        name: String,
        /// Bait value (the exposed/fake key)
        value: String,
    },
    
    /// List all secrets and honeypots
    List {
        /// Show honeypots only
        #[arg(long)]
        honeypots: bool,
        
        /// Show secrets only
        #[arg(long)]
        secrets: bool,
    },
    
    /// Show vault status
    Status,
    
    /// Export environment script
    Export {
        /// Output format: bash, powershell, fish
        #[arg(short, long, default_value = "bash")]
        format: String,
    },
    
    /// View honeypot alerts
    Alerts {
        /// Number of recent alerts to show
        #[arg(short, long, default_value = "10")]
        count: usize,
    },
    
    /// Deploy standard Strategickhaos honeypots
    DeployTraps {
        /// Include the exposed keys from environment scan
        #[arg(long)]
        from_env: bool,
    },
}

fn main() {
    let cli = Cli::parse();
    
    let mut vault = match FlameVault::new() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("❌ Failed to initialize FlameVault: {}", e);
            std::process::exit(1);
        }
    };
    
    match cli.command {
        Commands::Init => {
            println!("{}", vault.status());
            println!("✅ FlameVault initialized successfully!");
        }
        
        Commands::Set { name, value } => {
            match vault.set_secret(&name, &value) {
                Ok(_) => println!("✅ Secret '{}' encrypted and stored", name),
                Err(e) => eprintln!("❌ Failed to store secret: {}", e),
            }
        }
        
        Commands::Get { name, raw } => {
            match vault.get_secret(&name) {
                SecretResult::Real(value) => {
                    if raw {
                        print!("{}", value);
                    } else {
                        println!("🔓 {}: {}", name, value);
                    }
                }
                SecretResult::Honeypot(value) => {
                    if raw {
                        print!("{}", value);
                    } else {
                        println!("🍯 HONEYPOT: {} (access logged)", name);
                        println!("   Value: {}", value);
                    }
                }
                SecretResult::NotFound => {
                    eprintln!("❌ Secret '{}' not found", name);
                    std::process::exit(1);
                }
                SecretResult::DecryptionFailed => {
                    eprintln!("❌ Decryption failed - wrong machine?");
                    std::process::exit(1);
                }
            }
        }
        
        Commands::Honeypot { name, value } => {
            match vault.set_honeypot(&name, &value) {
                Ok(_) => {
                    println!("🍯 Honeypot '{}' deployed", name);
                    println!("   Any access will be logged to alerts");
                }
                Err(e) => eprintln!("❌ Failed to deploy honeypot: {}", e),
            }
        }
        
        Commands::List { honeypots, secrets } => {
            let show_all = !honeypots && !secrets;
            
            if show_all || secrets {
                println!("\n🔐 ENCRYPTED SECRETS:");
                match vault.list_secrets() {
                    Ok(list) => {
                        if list.is_empty() {
                            println!("   (none)");
                        } else {
                            for name in list {
                                println!("   • {}", name);
                            }
                        }
                    }
                    Err(e) => eprintln!("   Error: {}", e),
                }
            }
            
            if show_all || honeypots {
                println!("\n🍯 HONEYPOTS:");
                let pots = vault.list_honeypots();
                if pots.is_empty() {
                    println!("   (none)");
                } else {
                    for pot in pots {
                        println!("   • {} (accessed {} times)", pot.name, pot.access_count);
                    }
                }
            }
        }
        
        Commands::Status => {
            println!("{}", vault.status());
        }
        
        Commands::Export { format } => {
            match vault.export_env_script() {
                Ok(script) => {
                    let formatted = match format.as_str() {
                        "powershell" | "ps1" => script
                            .replace("export ", "$env:")
                            .replace("=\"", " = \""),
                        "fish" => script.replace("export ", "set -x "),
                        _ => script, // bash is default
                    };
                    println!("{}", formatted);
                }
                Err(e) => eprintln!("❌ Export failed: {}", e),
            }
        }
        
        Commands::Alerts { count } => {
            let alert_file = vault.vault_path.join("alerts.log");
            
            if !alert_file.exists() {
                println!("📭 No alerts yet");
                return;
            }
            
            match std::fs::read_to_string(&alert_file) {
                Ok(content) => {
                    let lines: Vec<&str> = content.lines().collect();
                    let recent: Vec<&str> = lines.iter().rev().take(count).copied().collect();
                    
                    println!("🚨 RECENT HONEYPOT ALERTS ({} of {}):\n", recent.len(), lines.len());
                    
                    for line in recent.iter().rev() {
                        if let Ok(alert) = serde_json::from_str::<serde_json::Value>(line) {
                            println!("  {} | {} | {} | {}",
                                alert["timestamp"].as_str().unwrap_or("?"),
                                alert["key_name"].as_str().unwrap_or("?"),
                                alert["device_id"].as_str().unwrap_or("?"),
                                alert["process_name"].as_str().unwrap_or("?")
                            );
                        }
                    }
                }
                Err(e) => eprintln!("❌ Failed to read alerts: {}", e),
            }
        }
        
        Commands::DeployTraps { from_env } => {
            println!("🍯 Deploying Strategickhaos honeypot traps...\n");
            
            // Standard honeypots
            let traps = vec![
                ("OPENAI_API_KEY", "sk-honeypot-strategickhaos-trap-do-not-use"),
                ("XAI_API_KEY", "xai-honeypot-strategickhaos-trap-do-not-use"),
                ("ANTHROPIC_API_KEY", "sk-ant-honeypot-strategickhaos-trap"),
                ("AWS_SECRET_ACCESS_KEY", "honeypot/strategickhaos/trap/key"),
                ("GITHUB_TOKEN", "ghp_honeypotStrategickhaosDoNotUse000000"),
                ("DATABASE_URL", "postgresql://honeypot:trap@localhost:5432/fake"),
            ];
            
            for (name, value) in traps {
                if let Err(e) = vault.set_honeypot(name, value) {
                    eprintln!("⚠️ Failed to set {}: {}", name, e);
                } else {
                    println!("   ✅ {}", name);
                }
            }
            
            if from_env {
                println!("\n   Scanning environment for exposed keys...");
                
                let env_keys = ["OPENAI_API_KEY", "XAI_API_KEY", "EMAIL_PASS", "GH_TOKEN"];
                
                for key in env_keys {
                    if let Ok(value) = std::env::var(key) {
                        if !value.contains("honeypot") {
                            println!("   🔍 Found: {} (converting to honeypot)", key);
                            let _ = vault.set_honeypot(&format!("ENV_{}", key), &value);
                        }
                    }
                }
            }
            
            println!("\n🍯 Honeypot traps deployed!");
            println!("   Any access will be logged to ~/.flamevault/alerts.log");
        }
    }
}
