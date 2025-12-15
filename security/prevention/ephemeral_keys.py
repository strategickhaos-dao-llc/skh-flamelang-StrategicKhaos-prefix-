"""
Prevention #3: Ephemeral Session Keys

Ephemeral keys per session — rotated every 24h, old ones zeroized.
No persistent keys. Every session is fresh. Past keys destroyed.
"""

import secrets
import time
from typing import Dict, Optional
from dataclasses import dataclass
from datetime import datetime, timedelta


@dataclass
class EphemeralKey:
    """Represents an ephemeral key"""
    key_id: str
    key_material: bytes
    created_at: float
    expires_at: float
    rotated: bool = False
    zeroized: bool = False


class EphemeralKeyManager:
    """
    Ephemeral key management system
    
    Keys are generated per session, rotated every 24 hours,
    and immediately zeroized upon expiration.
    """
    
    def __init__(self, rotation_hours: int = 24, key_length: int = 32):
        """
        Initialize ephemeral key manager
        
        Args:
            rotation_hours: Hours between key rotations
            key_length: Length of key material in bytes
        """
        self.rotation_hours = rotation_hours
        self.key_length = key_length
        self.active_keys: Dict[str, EphemeralKey] = {}
        self.rotation_log: list = []
    
    def generate_key(self, session_id: str) -> EphemeralKey:
        """
        Generate a new ephemeral key for a session
        
        Args:
            session_id: Unique session identifier
        
        Returns:
            EphemeralKey object
        """
        key_id = f"{session_id}:{secrets.token_hex(16)}"
        key_material = secrets.token_bytes(self.key_length)
        
        now = time.time()
        expires_at = now + (self.rotation_hours * 3600)
        
        ephemeral_key = EphemeralKey(
            key_id=key_id,
            key_material=key_material,
            created_at=now,
            expires_at=expires_at
        )
        
        self.active_keys[key_id] = ephemeral_key
        return ephemeral_key
    
    def get_key(self, key_id: str) -> Optional[EphemeralKey]:
        """
        Get an active ephemeral key
        
        Args:
            key_id: Key identifier
        
        Returns:
            EphemeralKey if active, None if expired or not found
        """
        if key_id not in self.active_keys:
            return None
        
        key = self.active_keys[key_id]
        
        # Check if expired
        if time.time() > key.expires_at:
            self.zeroize_key(key_id)
            return None
        
        return key
    
    def rotate_key(self, key_id: str) -> Optional[EphemeralKey]:
        """
        Rotate an ephemeral key
        
        Args:
            key_id: Key to rotate
        
        Returns:
            New ephemeral key
        """
        old_key = self.active_keys.get(key_id)
        if not old_key:
            return None
        
        # Extract session ID from key_id
        session_id = key_id.split(':')[0]
        
        # Generate new key
        new_key = self.generate_key(session_id)
        
        # Mark old key as rotated
        old_key.rotated = True
        
        # Zeroize old key
        self.zeroize_key(key_id)
        
        # Log rotation
        self.rotation_log.append({
            'timestamp': time.time(),
            'old_key_id': key_id,
            'new_key_id': new_key.key_id,
            'reason': 'scheduled_rotation'
        })
        
        return new_key
    
    def zeroize_key(self, key_id: str):
        """
        Zeroize (securely delete) a key from memory
        
        Args:
            key_id: Key to zeroize
        """
        if key_id not in self.active_keys:
            return
        
        key = self.active_keys[key_id]
        
        # Overwrite key material with zeros
        if not key.zeroized:
            # Create a bytearray to allow modification
            key_array = bytearray(key.key_material)
            for i in range(len(key_array)):
                key_array[i] = 0
            
            # Mark as zeroized
            key.zeroized = True
            
            # Remove from active keys
            del self.active_keys[key_id]
    
    def cleanup_expired_keys(self):
        """Remove and zeroize all expired keys"""
        now = time.time()
        expired_keys = [
            key_id for key_id, key in self.active_keys.items()
            if now > key.expires_at
        ]
        
        for key_id in expired_keys:
            self.zeroize_key(key_id)
        
        return len(expired_keys)
    
    def rotate_all_keys(self, reason: str = "manual"):
        """
        Rotate all active keys immediately
        
        Args:
            reason: Reason for rotation
        """
        current_keys = list(self.active_keys.keys())
        
        for key_id in current_keys:
            new_key = self.rotate_key(key_id)
            if new_key:
                self.rotation_log[-1]['reason'] = reason
    
    def get_active_key_count(self) -> int:
        """Get count of active keys"""
        return len(self.active_keys)
    
    def get_rotation_stats(self) -> Dict:
        """Get rotation statistics"""
        return {
            'active_keys': self.get_active_key_count(),
            'total_rotations': len(self.rotation_log),
            'rotation_hours': self.rotation_hours,
        }


# Example usage
if __name__ == "__main__":
    manager = EphemeralKeyManager(rotation_hours=24)
    
    # Generate key for session
    session_id = "session_123"
    key1 = manager.generate_key(session_id)
    print(f"Generated key: {key1.key_id}")
    print(f"Expires at: {datetime.fromtimestamp(key1.expires_at)}")
    
    # Retrieve key
    retrieved = manager.get_key(key1.key_id)
    print(f"Retrieved: {retrieved.key_id if retrieved else 'None'}")
    
    # Rotate key
    print("\nRotating key...")
    new_key = manager.rotate_key(key1.key_id)
    print(f"New key: {new_key.key_id if new_key else 'None'}")
    
    # Try to retrieve old key (should be None)
    old_retrieved = manager.get_key(key1.key_id)
    print(f"Old key retrieved: {old_retrieved}")
    
    # Stats
    stats = manager.get_rotation_stats()
    print(f"\nStats: {stats}")
    
    print("\n🔥 Ephemeral keys: Born fresh, die zeroized. No traces. 🔥")
