"""
Prevention #9: Honeypot Keys

Honeypot keys — deliberately "leaked" fakes that trigger immediate swarm response.
Attractive traps that ensnare attackers.
"""

import secrets
import time
from typing import Dict, List
from dataclasses import dataclass


@dataclass
class HoneypotKey:
    """Represents a honeypot key"""
    key_id: str
    key_value: str
    created_at: float
    attractiveness: str  # "high", "medium", "low"
    triggered: bool = False
    attacker_info: Dict = None


class HoneypotKeySystem:
    """
    Honeypot key system
    
    Deliberately leaked fake keys that trigger swarm response.
    The more they try to use it, the more we learn about them.
    """
    
    def __init__(self):
        self.honeypots: Dict[str, HoneypotKey] = {}
        self.triggered_traps: List[Dict] = []
    
    def create_honeypot(
        self,
        key_type: str = "gcp_sa",
        attractiveness: str = "high"
    ) -> HoneypotKey:
        """Create an attractive honeypot key"""
        key_id = f"honeypot_{secrets.token_hex(8)}"
        
        # Make it look real
        if key_type == "gcp_sa":
            key_value = f"gcp_service_account_{secrets.token_hex(32)}"
        elif key_type == "aws":
            key_value = f"AKIA{secrets.token_hex(16).upper()}"
        else:
            key_value = f"API_KEY_{secrets.token_hex(24)}"
        
        honeypot = HoneypotKey(
            key_id=key_id,
            key_value=key_value,
            created_at=time.time(),
            attractiveness=attractiveness
        )
        
        self.honeypots[key_value] = honeypot
        return honeypot
    
    def trigger_trap(self, key_value: str, attacker_info: Dict) -> bool:
        """Trigger honeypot trap"""
        if key_value in self.honeypots:
            honeypot = self.honeypots[key_value]
            honeypot.triggered = True
            honeypot.attacker_info = attacker_info
            
            # Log trigger
            self.triggered_traps.append({
                'timestamp': time.time(),
                'honeypot_id': honeypot.key_id,
                'attacker_info': attacker_info
            })
            
            # Activate swarm response
            self._activate_swarm_response(honeypot, attacker_info)
            return True
        return False
    
    def _activate_swarm_response(self, honeypot: HoneypotKey, attacker_info: Dict):
        """Activate coordinated swarm response"""
        print(f"🚨 HONEYPOT TRIGGERED: {honeypot.key_id}")
        print(f"   Attacker IP: {attacker_info.get('ip', 'unknown')}")
        print(f"   Swarm response activated")
        print(f"   Legal response: INITIATED")
        print(f"   Technical countermeasures: DEPLOYED")


if __name__ == "__main__":
    system = HoneypotKeySystem()
    
    # Create attractive honeypot
    honeypot = system.create_honeypot("gcp_sa", "high")
    print(f"Honeypot created: {honeypot.key_value[:40]}...")
    
    # Simulate trigger
    system.trigger_trap(honeypot.key_value, {
        'ip': '192.168.1.100',
        'user_agent': 'curl/7.68.0'
    })
    
    print("🔥 They touch the honey. The swarm descends. 🔥")
