import os
from typing import Any, Dict, cast
import jwt
from aws_lambda_powertools import Logger
from aws_lambda_powertools.logging import correlation_paths
from aws_lambda_powertools.utilities.typing import LambdaContext
from aws_lambda_powertools.utilities.data_classes.api_gateway_authorizer_event import APIGatewayAuthorizerTokenEvent
from jwt.algorithms import AllowedPublicKeys
from .key_storage import KeyStorage
from .principal_id_claims import PrincipalIDClaims
from .authorizer_response import allow, deny

# NOTE: this implementation only supports RSA, so it does not check for unsupported algorithms
JWKS_URI = os.environ["JWKS_URI"] # required
JWT_AUDIENCE = os.environ.get("JWT_AUDIENCE", None)
JWT_ISSUER = os.environ.get("JWT_ISSUER", None)
MIN_REFRESH_RATE = int(os.environ.get("MIN_REFRESH_RATE", 900)) # 15 minutes
PRINCIPAL_ID_CLAIMS = os.environ.get("PRINCIPAL_ID_CLAIMS", "preferred_username,sub")
DEFAULT_PRINCIPAL_ID = os.environ.get("DEFAULT_PRINCIPAL_ID", "unknown")

logger = Logger()
key_storage = KeyStorage(JWKS_URI, MIN_REFRESH_RATE)
principal_id_claims = PrincipalIDClaims.from_comma_separated_values(PRINCIPAL_ID_CLAIMS, DEFAULT_PRINCIPAL_ID)


def _validate_token(token: str) -> Dict[str, Any]:
    header = jwt.get_unverified_header(token)
    kid = header["kid"]
    algorithm = header["alg"]
    key = key_storage.get(kid)
    decoded = jwt.decode(
        token,
        cast(AllowedPublicKeys, key),
        algorithms=[algorithm],
        issuer=JWT_ISSUER,
        audience=JWT_AUDIENCE,
    )
    return decoded


@logger.inject_lambda_context(
    log_event=True, correlation_id_path=correlation_paths.API_GATEWAY_REST
)
def handler(event: APIGatewayAuthorizerTokenEvent, _context: LambdaContext) -> Dict[str, Any]:
    
    try:
        token = str.split(event["authorizationToken"])[1]
        
        token_claims = _validate_token(token)
        principal_id = principal_id_claims.get_principal_id_from_claims(token_claims)
        return allow(principal_id, token_claims)
    except BaseException as error:
        logger.error(f"An error occurred: {error}")
        return deny(event["methodArn"])