//! FlameVault CLI — Honeypot + Encrypted Secrets Engine
//! 
//! Command-line interface for managing encrypted secrets and honeypot traps

use clap::{Parser, Subcommand};
use flamelang::vault::FlameVault;

#[derive(Parser)]
#[command(name = "flamevault")]
#[command(about = "🔥 FlameVault — Honeypot + Encrypted Secrets Engine", long_about = None)]
#[command(version = "2.0.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize FlameVault
    Init,
    
    /// Store an encrypted secret
    Set {
        /// Secret name
        name: String,
        /// Secret value
        value: String,
    },
    
    /// Retrieve a secret (decrypts real secrets, logs honeypot access)
    Get {
        /// Secret name
        name: String,
        /// Output raw value without formatting
        #[arg(long)]
        raw: bool,
    },
    
    /// Deploy a honeypot trap
    Honeypot {
        /// Honeypot name
        name: String,
        /// Honeypot value (bait)
        value: String,
    },
    
    /// Deploy standard honeypot traps from environment variables
    DeployTraps {
        /// Load from environment variables
        #[arg(long)]
        from_env: bool,
    },
    
    /// List all secrets and honeypots
    List,
    
    /// View access alerts
    Alerts {
        /// Show only recent alerts
        #[arg(long)]
        recent: bool,
    },
    
    /// Show vault status
    Status,
}

fn main() {
    let cli = Cli::parse();
    
    if let Err(e) = run_command(cli.command) {
        eprintln!("❌ Error: {}", e);
        std::process::exit(1);
    }
}

fn run_command(command: Commands) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        Commands::Init => {
            let vault = FlameVault::new()?;
            let status = vault.status()?;
            println!("🔥 FlameVault initialized");
            println!("   Vault directory: {}", status.vault_dir.display());
            println!("   Machine ID: {}", status.machine_id);
            println!("\n✓ Ready to store encrypted secrets and deploy honeypots");
        }
        
        Commands::Set { name, value } => {
            let vault = FlameVault::new()?;
            vault.set_secret(&name, &value)?;
            println!("✓ Secret '{}' encrypted and stored", name);
        }
        
        Commands::Get { name, raw } => {
            let vault = FlameVault::new()?;
            let value = vault.get_secret(&name)?;
            
            if raw {
                print!("{}", value);
            } else {
                println!("✓ Secret '{}': {}", name, value);
            }
        }
        
        Commands::Honeypot { name, value } => {
            let vault = FlameVault::new()?;
            vault.set_honeypot(&name, &value)?;
            println!("🍯 Honeypot '{}' deployed", name);
        }
        
        Commands::DeployTraps { from_env } => {
            let vault = FlameVault::new()?;
            
            if from_env {
                let deployed = vault.deploy_traps()?;
                
                if deployed.is_empty() {
                    println!("⚠️  No environment variables found to deploy as honeypots");
                } else {
                    println!("🍯 Deployed {} honeypot traps:", deployed.len());
                    for name in deployed {
                        println!("   • {}", name);
                    }
                }
            } else {
                println!("Use --from-env to deploy honeypots from environment variables");
            }
        }
        
        Commands::List => {
            let vault = FlameVault::new()?;
            let secrets = vault.list_secrets()?;
            
            if secrets.is_empty() {
                println!("No secrets or honeypots configured");
            } else {
                println!("📋 FlameVault contents:");
                for secret in secrets {
                    println!("   • {}", secret);
                }
            }
        }
        
        Commands::Alerts { recent } => {
            let vault = FlameVault::new()?;
            let mut alerts = vault.get_alerts()?;
            
            if recent {
                // Show only last 10 alerts
                alerts = alerts.into_iter().rev().take(10).collect();
            }
            
            if alerts.is_empty() {
                println!("✓ No alerts (no honeypot access detected)");
            } else {
                println!("🚨 Access alerts ({} total):", alerts.len());
                for alert in alerts.iter().rev() {
                    println!("   [{}] Honeypot '{}' accessed by {} (PID: {}, User: {})",
                        alert.timestamp.format("%Y-%m-%d %H:%M:%S"),
                        alert.honeypot_name,
                        alert.process_name,
                        alert.process_id,
                        alert.user
                    );
                }
            }
        }
        
        Commands::Status => {
            let vault = FlameVault::new()?;
            let status = vault.status()?;
            
            println!("🔥 FlameVault Status");
            println!("   ├─ Vault directory: {}", status.vault_dir.display());
            println!("   ├─ Machine ID: {}", status.machine_id);
            println!("   ├─ Encrypted secrets: {}", status.secret_count);
            println!("   ├─ Honeypot traps: {}", status.honeypot_count);
            println!("   └─ Access alerts: {}", status.alert_count);
        }
    }
    
    Ok(())
}
