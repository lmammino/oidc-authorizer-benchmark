import json
from typing import Any, Dict

def allow(principal_id: str, token_claims: Dict[str, Any]) -> Dict[str, Any]:
  return {
    "context": {
      "claims": json.dumps(token_claims),
    },
    "principal_id": principal_id,
    "policy_document": {
      "Version": "2012-10-17",
      "Statement": [
        {
          "Effect": "Allow",
          "Action": "execute-api:Invoke",
          "Resource": "*",
        },
      ],
    }
  }

def deny(resource: str) -> Dict[str, Any]:
  return {
    "policy_document": {
      "Version": "2012-10-17",
      "Statement": [
        {
          "Effect": "Deny",
          "Action": "execute-api:Invoke",
          "Resource": resource,
        },
      ],
    }
  }