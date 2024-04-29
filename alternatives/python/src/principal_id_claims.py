from typing import Dict, List, Any


class PrincipalIDClaims:
  def __init__(self, fields: List[str], default_value: str):
    self.fields = fields
    self.default_value = default_value

  @staticmethod
  def from_comma_separated_values(comma_separated_values: str, default_value: str) -> 'PrincipalIDClaims':
    fields = list(map(str.strip, comma_separated_values.split(',')))
    return PrincipalIDClaims(fields, default_value)
  

  def get_principal_id_from_claims(self, claims: Dict[str, Any]) -> str:
    for field in self.fields:
      if field in claims:
        return str(claims[field])
    return self.default_value