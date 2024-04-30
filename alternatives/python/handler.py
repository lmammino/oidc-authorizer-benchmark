import os
from typing import Any, Dict
import jwt
from aws_lambda_powertools import Logger
from aws_lambda_powertools.logging import correlation_paths
from aws_lambda_powertools.utilities.typing import LambdaContext
from aws_lambda_powertools.utilities.data_classes.api_gateway_authorizer_event import APIGatewayAuthorizerTokenEvent
from utils.header_parser import get_token_from_header
from utils.key_storage import KeyStorage
from utils.principal_id_claims import PrincipalIDClaims
from utils.authorizer_response import allow, deny

JWKS_URI = os.environ["JWKS_URI"] # required
ACCEPTED_AUDIENCES = os.environ.get("ACCEPTED_AUDIENCES", None)
ACCEPTED_ISSUER = os.environ.get("ACCEPTED_ISSUER", None)
MIN_REFRESH_RATE = int(os.environ.get("MIN_REFRESH_RATE", 900)) # 15 minutes
PRINCIPAL_ID_CLAIMS = os.environ.get("PRINCIPAL_ID_CLAIMS", "preferred_username,sub")
DEFAULT_PRINCIPAL_ID = os.environ.get("DEFAULT_PRINCIPAL_ID", "unknown")


accepted_audiences = list(map(str.strip, ACCEPTED_AUDIENCES.split(","))) if ACCEPTED_AUDIENCES else None
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
        key,
        algorithms=[algorithm],
        issuer=ACCEPTED_ISSUER,
        audience=accepted_audiences,
    )
    return decoded


@logger.inject_lambda_context(
    log_event=True, correlation_id_path=correlation_paths.API_GATEWAY_REST
)
def handler(event: APIGatewayAuthorizerTokenEvent, _context: LambdaContext) -> Dict[str, Any]:
    try:
        token = get_token_from_header(event["authorizationToken"])
        token_claims = _validate_token(token)
        principal_id = principal_id_claims.get_principal_id_from_claims(token_claims)
        logger.debug(f"Successfully authenticated Principal ID: {principal_id}", principal_id=principal_id)
        return allow(principal_id, token_claims)
    except BaseException as error:
        logger.error(f"An error occurred: {error}")
        return deny(event["methodArn"])