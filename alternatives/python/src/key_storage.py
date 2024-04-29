from datetime import datetime, timedelta, timezone
from typing import Dict
import requests
from jwt.algorithms import RSAAlgorithm, AllowedRSAKeys

class KeyNotFoundError(Exception):
  def __init__(self, key_id: str):
    super().__init__(f"Key not found: {key_id}")
    self.key_id = key_id

class KeyStorage:
  def __init__(self, jwks_uri: str, min_refresh_rate: int):
    self.jwks_uri = jwks_uri
    self.min_refresh_rate = min_refresh_rate
    self.last_refresh_time: None | datetime = None
    self.cached_jwk: Dict[str, AllowedRSAKeys] = {}

  def get(self, key_id: str) -> AllowedRSAKeys:
    key = self.cached_jwk.get(key_id)
    if key:
      return key
    
    now = datetime.now(tz=timezone.utc)
    should_refresh = self.last_refresh_time is None or self.last_refresh_time < now - timedelta(seconds=self.min_refresh_rate)
    if should_refresh:
      raw_jwks = requests.get(self.jwks_uri).json()
      for jwk in raw_jwks["keys"]:
        kid = str(jwk["kid"])
        self.cached_jwk[kid] = RSAAlgorithm.from_jwk(jwk)
      self.last_refresh_time = now

    key = self.cached_jwk.get(key_id)
    if key:
      return key
    
    raise KeyNotFoundError(key_id)