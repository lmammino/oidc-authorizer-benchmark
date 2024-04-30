class InvalidAuthorizationHeader(Exception):
  def __init__(self, header_value: str):
    super().__init__(f"Authorization header does not start with 'Bearer '. Given header value: '{header_value}'")
    self.header_value = header_value

def get_token_from_header(authorization_header: str) -> str:
  if not authorization_header.startswith("Bearer "):
    raise InvalidAuthorizationHeader(authorization_header)
  return str.split(authorization_header)[1]