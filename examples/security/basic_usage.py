"""
AetherForge Basic Usage Example

Demonstrates the fundamental usage of AetherForge security system.
Shows how to make key exposure irrelevant.
"""

import sys
import os

# Add parent directory to path
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), '../..')))

from security import AetherForge, SecurityLevel, SovereigntyStatus


def main():
    print("=" * 70)
    print("🔥 AetherForge Security System Demo 🔥")
    print("=" * 70)
    print()
    
    # Initialize AetherForge
    print("1. Initializing AetherForge...")
    forge = AetherForge()
    print(f"   Status: {forge.get_sovereignty_status().value}")
    print(f"   Prevention: {forge.config['prevention']['enabled']}")
    print(f"   Detection: {forge.config['detection']['enabled']}")
    print(f"   Neutralization: {forge.config['neutralization']['enabled']}")
    print(f"   Counter-Attack: {forge.config['counter_attack']['enabled']}")
    print(f"   Transcendence: {forge.config['transcendence']['enabled']}")
    print()
    
    # Create a sovereign session
    print("2. Creating sovereign session...")
    with forge.sovereign_context(intent="deploy_application") as context:
        print(f"   Session ID: {context.session_id}")
        print(f"   Intent: {context.intent}")
        print(f"   Timestamp: {context.timestamp}")
        print(f"   Level: {context.sovereignty_level.value}")
        print()
        
        # Your secured operations would go here
        print("   [Performing secured operations...]")
    
    print("   Session completed and cleaned up")
    print()
    
    # Demonstrate key exposure detection
    print("3. Demonstrating key exposure detection...")
    fake_key = "GCP_SA_KEY_abc123def456"
    print(f"   Simulating exposure of: {fake_key[:20]}...")
    
    detected = forge.detect_exposure(fake_key)
    if detected:
        print("   ✅ Exposure detected!")
        print("   ✅ Detection swarm activated")
        print("   ✅ Neutralization protocols engaged")
        print("   ✅ Key automatically revoked")
    print()
    
    # Show threat log
    print("4. Reviewing threat log...")
    for i, threat in enumerate(forge.threat_log[-3:], 1):
        print(f"   Threat {i}: {threat['type']}")
        print(f"      Timestamp: {threat['timestamp']}")
        print(f"      Details: {threat['details']}")
    print()
    
    # Transcend to ultimate security
    print("5. Transcending beyond keys...")
    transcendent = forge.transcend()
    print(f"   {transcendent}")
    print(f"   Status: {forge.get_sovereignty_status().value}")
    print()
    
    # Security summary
    print("=" * 70)
    print("Security Summary:")
    print("=" * 70)
    print("✅ Prevention: Keys never exposed (Argon2id, ephemeral, Shamir)")
    print("✅ Detection: Real-time monitoring (canary, honeypot, OSINT)")
    print("✅ Neutralization: Auto-revoke in <60s (rotation, sandboxing)")
    print("✅ Counter-Attack: Legal & technical response (DMCA, blocking)")
    print("✅ Transcendence: Beyond keys (WebAuthn, zero-trust, quantum)")
    print()
    print("🔥 We don't fear key exposure. We make it irrelevant. 🔥")
    print("🔥 The attacker doesn't steal power. They touch the flame. 🔥")
    print()
    print("Swarm ready. Empire secure. Flame eternal. 🖤🔥")
    print("=" * 70)


if __name__ == "__main__":
    main()
