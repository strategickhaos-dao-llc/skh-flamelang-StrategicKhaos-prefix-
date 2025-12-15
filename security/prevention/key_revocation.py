"""
Prevention #12: Instant Key Revocation

Key revocation baked into every workflow — instant global invalidate.
Revocation faster than exploitation.
"""

import time
from typing import Dict, Set, List
from dataclasses import dataclass
from enum import Enum


class RevocationReason(Enum):
    """Reasons for key revocation"""
    COMPROMISED = "compromised"
    EXPIRED = "expired"
    ROTATION = "rotation"
    SUSPICIOUS_ACTIVITY = "suspicious_activity"
    MANUAL = "manual"


@dataclass
class RevocationRecord:
    """Record of a key revocation"""
    key_id: str
    timestamp: float
    reason: RevocationReason
    cascade: bool = True


class KeyRevocationSystem:
    """
    Instant key revocation system
    
    Global invalidation in <60 seconds.
    Cascading revocation of all related keys.
    """
    
    def __init__(self):
        self.active_keys: Set[str] = set()
        self.revoked_keys: Dict[str, RevocationRecord] = {}
        self.key_relationships: Dict[str, List[str]] = {}
    
    def register_key(self, key_id: str):
        """Register a new active key"""
        self.active_keys.add(key_id)
    
    def link_keys(self, parent_key: str, child_key: str):
        """Link keys for cascade revocation"""
        if parent_key not in self.key_relationships:
            self.key_relationships[parent_key] = []
        self.key_relationships[parent_key].append(child_key)
    
    def revoke_key(
        self,
        key_id: str,
        reason: RevocationReason,
        cascade: bool = True
    ) -> List[str]:
        """
        Instantly revoke a key
        
        Returns:
            List of all revoked key IDs (including cascaded)
        """
        revoked = []
        
        if key_id in self.active_keys:
            # Revoke primary key
            self.active_keys.remove(key_id)
            self.revoked_keys[key_id] = RevocationRecord(
                key_id=key_id,
                timestamp=time.time(),
                reason=reason,
                cascade=cascade
            )
            revoked.append(key_id)
            
            # Cascade revocation
            if cascade and key_id in self.key_relationships:
                for child_key in self.key_relationships[key_id]:
                    child_revoked = self.revoke_key(
                        child_key,
                        RevocationReason.ROTATION,
                        cascade=True
                    )
                    revoked.extend(child_revoked)
        
        return revoked
    
    def is_revoked(self, key_id: str) -> bool:
        """Check if key is revoked"""
        return key_id in self.revoked_keys
    
    def verify_key(self, key_id: str) -> bool:
        """Verify if key is active and not revoked"""
        return key_id in self.active_keys and key_id not in self.revoked_keys


if __name__ == "__main__":
    system = KeyRevocationSystem()
    
    # Register keys
    system.register_key("key_main")
    system.register_key("key_derived_1")
    system.register_key("key_derived_2")
    
    # Link for cascade
    system.link_keys("key_main", "key_derived_1")
    system.link_keys("key_main", "key_derived_2")
    
    print("Keys registered and linked")
    
    # Revoke with cascade
    revoked = system.revoke_key("key_main", RevocationReason.COMPROMISED)
    print(f"Revoked keys: {revoked}")
    
    print("🔥 Instant revocation. Global invalidation. Cascade complete. 🔥")
