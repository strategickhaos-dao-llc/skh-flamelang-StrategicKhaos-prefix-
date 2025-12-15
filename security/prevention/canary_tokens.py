"""
Prevention #8: Canary Tokens

Canary tokens in fake keys — alert on access attempt.
Tripwires that detect unauthorized access immediately.
"""

import secrets
import time
from typing import Dict, Callable, Optional
from dataclasses import dataclass


@dataclass
class CanaryToken:
    """Represents a canary token"""
    token_id: str
    token_value: str
    created_at: float
    accessed: bool = False
    access_count: int = 0


class CanaryTokenSystem:
    """
    Canary token detection system
    
    Fake keys that trigger alerts when accessed.
    Silent watchers that expose attackers.
    """
    
    def __init__(self):
        self.tokens: Dict[str, CanaryToken] = {}
        self.alert_callbacks: list = []
    
    def create_canary(self, key_type: str = "generic") -> CanaryToken:
        """Create a new canary token"""
        token_id = f"canary_{secrets.token_hex(8)}"
        token_value = f"{key_type.upper()}_CANARY_{secrets.token_hex(16)}"
        
        canary = CanaryToken(
            token_id=token_id,
            token_value=token_value,
            created_at=time.time()
        )
        
        self.tokens[token_value] = canary
        return canary
    
    def register_alert_callback(self, callback: Callable):
        """Register callback for canary access"""
        self.alert_callbacks.append(callback)
    
    def check_access(self, value: str) -> bool:
        """Check if value is a canary token"""
        if value in self.tokens:
            canary = self.tokens[value]
            canary.accessed = True
            canary.access_count += 1
            
            # Trigger alerts
            self._trigger_alerts(canary)
            return True
        return False
    
    def _trigger_alerts(self, canary: CanaryToken):
        """Trigger all alert callbacks"""
        for callback in self.alert_callbacks:
            callback(canary)


if __name__ == "__main__":
    system = CanaryTokenSystem()
    
    # Create canary
    canary = system.create_canary("gcp_key")
    print(f"Canary created: {canary.token_value[:30]}...")
    
    # Register alert
    system.register_alert_callback(
        lambda c: print(f"🚨 CANARY TRIGGERED: {c.token_id}")
    )
    
    # Simulate access
    system.check_access(canary.token_value)
    print("🔥 Canary tokens: Silent sentinels. 🔥")
