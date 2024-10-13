import os

DEV_ENV: bool = (
    (os.getenv("DETA_SPACE_APP") and os.getenv("DETA_SPACE_APP") != "true")  # type: ignore
    or (os.getenv("VIRTUAL_ENV") is not None)
    or (os.getenv("PIPENV_ACTIVE") and os.getenv("PIPENV_ACTIVE") == "1")
)
PROD_ENV: bool = not DEV_ENV
DEV_ENV = not PROD_ENV
