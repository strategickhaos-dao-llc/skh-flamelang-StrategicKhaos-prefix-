"""
Prevention #6: Shamir's Secret Sharing

Shamir's Secret Sharing across 3 nodes — 2-of-3 required, no single point.
Cryptographic threshold scheme: k-of-n shares needed to reconstruct secret.
"""

import secrets
from typing import List, Tuple
from dataclasses import dataclass


@dataclass
class SecretShare:
    """Represents a share of a secret"""
    share_id: int
    share_value: int
    node_id: str


class ShamirSecretSharing:
    """
    Shamir's Secret Sharing implementation
    
    Splits secrets into n shares where k shares are required to reconstruct.
    No single share reveals any information about the secret.
    """
    
    def __init__(self, threshold: int = 2, total_shares: int = 3, prime: int = None):
        """
        Initialize Shamir's Secret Sharing
        
        Args:
            threshold: Minimum shares needed (k)
            total_shares: Total shares to generate (n)
            prime: Prime number for modular arithmetic (auto-generated if None)
        """
        if threshold > total_shares:
            raise ValueError("Threshold cannot exceed total shares")
        
        self.threshold = threshold
        self.total_shares = total_shares
        # Use a very large prime for security (2^521 - 1, Mersenne prime)
        self.prime = prime or 6864797660130609714981900799081393217269435300143305409394463459185543183397656052122559640661454554977296311391480858037121987999716643812574028291115057151
    
    def _generate_polynomial(self, secret: int) -> List[int]:
        """
        Generate random polynomial with secret as constant term
        
        Args:
            secret: The secret (a0)
        
        Returns:
            List of coefficients [a0, a1, ..., a(k-1)]
        """
        coefficients = [secret]
        for _ in range(self.threshold - 1):
            coefficients.append(secrets.randbelow(self.prime))
        return coefficients
    
    def _evaluate_polynomial(self, coefficients: List[int], x: int) -> int:
        """
        Evaluate polynomial at point x
        
        Args:
            coefficients: Polynomial coefficients
            x: Point to evaluate at
        
        Returns:
            f(x) mod prime
        """
        result = 0
        for i, coef in enumerate(coefficients):
            result += coef * pow(x, i, self.prime)
            result %= self.prime
        return result
    
    def split_secret(self, secret: bytes, node_ids: List[str] = None) -> List[SecretShare]:
        """
        Split secret into shares
        
        Args:
            secret: Secret to split
            node_ids: Optional list of node identifiers
        
        Returns:
            List of SecretShare objects
        """
        # Convert secret bytes to integer
        secret_int = int.from_bytes(secret, byteorder='big')
        
        if secret_int >= self.prime:
            raise ValueError("Secret too large for prime")
        
        # Generate polynomial
        coefficients = self._generate_polynomial(secret_int)
        
        # Generate node IDs if not provided
        if node_ids is None:
            node_ids = [f"node_{i+1}" for i in range(self.total_shares)]
        elif len(node_ids) != self.total_shares:
            raise ValueError(f"Need {self.total_shares} node IDs")
        
        # Generate shares
        shares = []
        for i in range(1, self.total_shares + 1):
            share_value = self._evaluate_polynomial(coefficients, i)
            shares.append(SecretShare(
                share_id=i,
                share_value=share_value,
                node_id=node_ids[i-1]
            ))
        
        return shares
    
    def _lagrange_interpolation(self, shares: List[SecretShare]) -> int:
        """
        Reconstruct secret using Lagrange interpolation
        
        Args:
            shares: List of shares (at least threshold)
        
        Returns:
            Reconstructed secret
        """
        if len(shares) < self.threshold:
            raise ValueError(f"Need at least {self.threshold} shares")
        
        # Take only threshold shares
        shares = shares[:self.threshold]
        
        secret = 0
        for i, share_i in enumerate(shares):
            numerator = 1
            denominator = 1
            
            for j, share_j in enumerate(shares):
                if i != j:
                    numerator = (numerator * (-share_j.share_id)) % self.prime
                    denominator = (denominator * (share_i.share_id - share_j.share_id)) % self.prime
            
            # Modular multiplicative inverse
            lagrange_coefficient = (numerator * pow(denominator, -1, self.prime)) % self.prime
            secret = (secret + share_i.share_value * lagrange_coefficient) % self.prime
        
        return secret
    
    def reconstruct_secret(self, shares: List[SecretShare], secret_length: int) -> bytes:
        """
        Reconstruct secret from shares
        
        Args:
            shares: List of shares (at least threshold)
            secret_length: Expected length of secret in bytes
        
        Returns:
            Reconstructed secret
        """
        secret_int = self._lagrange_interpolation(shares)
        
        # Convert integer back to bytes
        secret_bytes = secret_int.to_bytes(secret_length, byteorder='big')
        return secret_bytes
    
    def verify_share(self, share: SecretShare) -> bool:
        """
        Verify that a share is valid (basic validation)
        
        Args:
            share: Share to verify
        
        Returns:
            True if valid
        """
        return (
            1 <= share.share_id <= self.total_shares and
            0 <= share.share_value < self.prime
        )


# Example usage
if __name__ == "__main__":
    # Initialize with 2-of-3 threshold
    shamir = ShamirSecretSharing(threshold=2, total_shares=3)
    
    # Secret to protect
    secret = b"GCP_SERVICE_ACCOUNT_KEY_SUPER_SECRET"
    print(f"Original secret: {secret.decode()}")
    print(f"Secret length: {len(secret)} bytes")
    
    # Define nodes
    nodes = ["node_us_east", "node_eu_west", "node_asia_pacific"]
    
    # Split secret
    shares = shamir.split_secret(secret, nodes)
    print(f"\n✂️  Split into {len(shares)} shares:")
    for share in shares:
        print(f"  {share.node_id}: Share #{share.share_id} = {share.share_value % 10000}...")
    
    # Reconstruct with 2 shares (threshold)
    print(f"\n🔄 Reconstructing with {shamir.threshold} shares...")
    reconstructed = shamir.reconstruct_secret(shares[:2], len(secret))
    print(f"Reconstructed: {reconstructed.decode()}")
    print(f"Match: {reconstructed == secret}")
    
    # Try with different combination
    print(f"\n🔄 Reconstructing with different 2 shares...")
    reconstructed2 = shamir.reconstruct_secret([shares[0], shares[2]], len(secret))
    print(f"Reconstructed: {reconstructed2.decode()}")
    print(f"Match: {reconstructed2 == secret}")
    
    # Try with only 1 share (should fail)
    print(f"\n❌ Attempting with only 1 share...")
    try:
        shamir.reconstruct_secret([shares[0]], len(secret))
    except ValueError as e:
        print(f"Failed as expected: {e}")
    
    print("\n🔥 No single point of failure. 2-of-3 sovereignty. 🔥")
