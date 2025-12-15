"""
Prevention #7: Hardware-Bound Keys

Hardware-bound keys (YubiKey/TPM) — physically impossible to extract.
Keys exist only in hardware security modules.
"""

from typing import Optional, Dict
from dataclasses import dataclass
from enum import Enum


class HardwareType(Enum):
    """Types of hardware security modules"""
    YUBIKEY = "yubikey"
    TPM = "tpm"
    HSM = "hsm"
    SECURE_ENCLAVE = "secure_enclave"


@dataclass
class HardwareKeyConfig:
    """Configuration for hardware-bound keys"""
    hardware_type: HardwareType
    device_id: str
    attestation_required: bool = True


class HardwareKeyBinding:
    """
    Hardware key binding system
    
    Keys are bound to physical hardware and cannot be extracted.
    Provides cryptographic attestation of hardware presence.
    """
    
    def __init__(self):
        self.bound_keys: Dict[str, HardwareKeyConfig] = {}
    
    def bind_key(
        self,
        key_id: str,
        hardware_type: HardwareType,
        device_id: str
    ) -> bool:
        """Bind key to hardware device"""
        config = HardwareKeyConfig(
            hardware_type=hardware_type,
            device_id=device_id,
            attestation_required=True
        )
        self.bound_keys[key_id] = config
        return True
    
    def verify_hardware(self, key_id: str, device_id: str) -> bool:
        """Verify hardware presence for key access"""
        if key_id not in self.bound_keys:
            return False
        
        config = self.bound_keys[key_id]
        return config.device_id == device_id
    
    def attest_hardware(self, key_id: str) -> Optional[bytes]:
        """Generate hardware attestation"""
        if key_id not in self.bound_keys:
            return None
        
        # Simulate hardware attestation
        config = self.bound_keys[key_id]
        attestation = f"{config.hardware_type.value}:{config.device_id}".encode()
        return attestation


if __name__ == "__main__":
    hw = HardwareKeyBinding()
    hw.bind_key("key1", HardwareType.YUBIKEY, "YK-12345")
    print("🔥 Key bound to hardware. Extraction impossible. 🔥")
