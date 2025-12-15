//! FlameVault CLI - Honeypot + Encrypted Secrets Manager
//! 
//! INV-080: Strategickhaos DAO LLC

use flamelang::flamevault::{FlameVault, FlameVaultResult};
use std::env;
use std::io::{self, Write};

fn main() -> FlameVaultResult<()> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_usage();
        return Ok(());
    }
    
    let vault = FlameVault::new()?;
    
    match args[1].as_str() {
        "set" => {
            if args.len() < 4 {
                eprintln!("Usage: flamevault set <name> <value>");
                std::process::exit(1);
            }
            let name = &args[2];
            let value = &args[3];
            vault.set_secret(name, value)?;
            println!("✓ Secret stored: {}", name);
        }
        
        "get" => {
            if args.len() < 3 {
                eprintln!("Usage: flamevault get <name>");
                std::process::exit(1);
            }
            let name = &args[2];
            match vault.get_secret(name) {
                Ok(value) => println!("{}", value),
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        
        "get-real" => {
            if args.len() < 3 {
                eprintln!("Usage: flamevault get-real <name>");
                std::process::exit(1);
            }
            let name = &args[2];
            match vault.get_real_secret(name) {
                Ok(value) => println!("{}", value),
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        
        "list" => {
            let secrets = vault.list_secrets()?;
            if secrets.is_empty() {
                println!("No secrets stored");
            } else {
                println!("Stored secrets:");
                for secret in secrets {
                    println!("  - {}", secret);
                }
            }
        }
        
        "remove" | "rm" => {
            if args.len() < 3 {
                eprintln!("Usage: flamevault remove <name>");
                std::process::exit(1);
            }
            let name = &args[2];
            vault.remove_secret(name)?;
            println!("✓ Secret removed: {}", name);
        }
        
        "deploy-honeypot" => {
            vault.deploy_honeypot()?;
            println!("✓ Honeypot deployed");
        }
        
        "alerts" => {
            let count = if args.len() >= 3 {
                args[2].parse().unwrap_or(10)
            } else {
                10
            };
            let alerts = vault.get_alerts(count)?;
            if alerts.is_empty() {
                println!("No alerts");
            } else {
                println!("Recent honeypot alerts:");
                for alert in alerts {
                    println!(
                        "[{}] {} - Key: {} (User: {}, Process: {})",
                        alert.timestamp, alert.alert, alert.key, alert.user, alert.process
                    );
                }
            }
        }
        
        "init" => {
            println!("🔥 FlameVault initialized at ~/.flamevault/");
            println!("Machine-bound encryption enabled");
        }
        
        "interactive" | "-i" => {
            interactive_mode(vault)?;
        }
        
        "help" | "--help" | "-h" => {
            print_usage();
        }
        
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            print_usage();
            std::process::exit(1);
        }
    }
    
    Ok(())
}

fn print_usage() {
    println!(r#"
🔥 FlameVault - Honeypot + Encrypted Secrets Manager
INV-080: Strategickhaos DAO LLC

USAGE:
    flamevault <COMMAND> [OPTIONS]

COMMANDS:
    set <name> <value>     Store an encrypted secret
    get <name>             Retrieve a secret (direct access)
    get-real <name>        Retrieve secret with honeypot routing
    list                   List all stored secrets
    remove <name>          Remove a secret
    deploy-honeypot        Deploy honeypot keys to environment
    alerts [count]         Show recent honeypot alerts (default: 10)
    init                   Initialize FlameVault
    interactive, -i        Interactive mode
    help, --help, -h       Show this help message

EXAMPLES:
    # Store a real API key (encrypted)
    flamevault set REAL_OPENAI_API_KEY "sk-your-actual-key"
    
    # Deploy honeypot bait
    flamevault deploy-honeypot
    
    # Retrieve with smart routing (checks honeypot first)
    flamevault get-real OPENAI_API_KEY
    
    # List stored secrets
    flamevault list
    
    # Check for honeypot alerts
    flamevault alerts 5

HONEYPOT KEYS:
    - OPENAI_API_KEY
    - XAI_API_KEY
    - EMAIL_PASS

Any access to honeypot keys will be logged as a security alert.
Real keys should be stored with the "REAL_" prefix.

VAULT LOCATION:
    ~/.flamevault/

ENCRYPTION:
    Machine-bound key derived from:
    - Hostname
    - STRAT_HEMISPHERE (env var)
    - STRAT_DEVICE_ID (env var)
    - Constant salt
"#);
}

fn interactive_mode(vault: FlameVault) -> FlameVaultResult<()> {
    println!("🔥 FlameVault Interactive Mode");
    println!("Type 'help' for commands, 'exit' to quit\n");
    
    loop {
        print!("flamevault> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        
        match parts[0] {
            "exit" | "quit" | "q" => {
                println!("Goodbye!");
                break;
            }
            "help" => {
                println!("Commands: set, get, get-real, list, remove, alerts, deploy-honeypot, exit");
            }
            "set" => {
                if parts.len() < 3 {
                    println!("Usage: set <name> <value>");
                    continue;
                }
                let name = parts[1];
                let value = parts[2..].join(" ");
                match vault.set_secret(name, &value) {
                    Ok(_) => println!("✓ Secret stored: {}", name),
                    Err(e) => println!("Error: {}", e),
                }
            }
            "get" => {
                if parts.len() < 2 {
                    println!("Usage: get <name>");
                    continue;
                }
                let name = parts[1];
                match vault.get_secret(name) {
                    Ok(value) => println!("{}", value),
                    Err(e) => println!("Error: {}", e),
                }
            }
            "get-real" => {
                if parts.len() < 2 {
                    println!("Usage: get-real <name>");
                    continue;
                }
                let name = parts[1];
                match vault.get_real_secret(name) {
                    Ok(value) => println!("{}", value),
                    Err(e) => println!("Error: {}", e),
                }
            }
            "list" => {
                match vault.list_secrets() {
                    Ok(secrets) => {
                        if secrets.is_empty() {
                            println!("No secrets stored");
                        } else {
                            println!("Stored secrets:");
                            for secret in secrets {
                                println!("  - {}", secret);
                            }
                        }
                    }
                    Err(e) => println!("Error: {}", e),
                }
            }
            "remove" | "rm" => {
                if parts.len() < 2 {
                    println!("Usage: remove <name>");
                    continue;
                }
                let name = parts[1];
                match vault.remove_secret(name) {
                    Ok(_) => println!("✓ Secret removed: {}", name),
                    Err(e) => println!("Error: {}", e),
                }
            }
            "alerts" => {
                let count = if parts.len() >= 2 {
                    parts[1].parse().unwrap_or(10)
                } else {
                    10
                };
                match vault.get_alerts(count) {
                    Ok(alerts) => {
                        if alerts.is_empty() {
                            println!("No alerts");
                        } else {
                            println!("Recent honeypot alerts:");
                            for alert in alerts {
                                println!(
                                    "[{}] {} - Key: {}",
                                    alert.timestamp, alert.alert, alert.key
                                );
                            }
                        }
                    }
                    Err(e) => println!("Error: {}", e),
                }
            }
            "deploy-honeypot" => {
                match vault.deploy_honeypot() {
                    Ok(_) => println!("✓ Honeypot deployed"),
                    Err(e) => println!("Error: {}", e),
                }
            }
            _ => {
                println!("Unknown command: {}", parts[0]);
                println!("Type 'help' for available commands");
            }
        }
    }
    
    Ok(())
}
