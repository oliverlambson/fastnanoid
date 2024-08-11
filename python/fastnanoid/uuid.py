import base64
import uuid


def uuid_to_urlid(uuid_: uuid.UUID) -> str:
    """Convert a UUID to a short URL-safe string.

    >>> import base64
    >>> import uuid
    >>> id_ = uuid.UUID('5d98d578-2731-4a4d-b666-70ca16f10aa2')
    >>> url_id = uuid_to_urlid(id_)
    >>> print(url_id)
    XZjVeCcxSk22ZnDKFvEKog
    """
    return base64.urlsafe_b64encode(uuid_.bytes).rstrip(b"=").decode("utf-8")


def urlid_to_uuid(url: str) -> uuid.UUID:
    """Convert a base64url encoded UUID string to a UUID.

    >>> import base64
    >>> import uuid
    >>> url_id = 'XZjVeCcxSk22ZnDKFvEKog'
    >>> id_ = urlid_to_uuid(url_id)
    >>> print(id_)
    5d98d578-2731-4a4d-b666-70ca16f10aa2
    """
    return uuid.UUID(bytes=base64.urlsafe_b64decode(url + "=" * (len(url) % 4)))
