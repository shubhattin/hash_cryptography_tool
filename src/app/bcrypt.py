"""
Page :- /bcrypt
"""
from fastapi import APIRouter, Request, Form
from fastapi.responses import PlainTextResponse
from kry.plugins import render_page
from ._router import app_router
from typing import Annotated
from kry.gupta import (
    bcrypt_salt,
    bcrypt_verify,
    bcrypt_hash,
)

router = APIRouter(prefix="", default_response_class=PlainTextResponse)


@router.get("/bcrypt")
async def bcrypt_page(req: Request):
    return render_page("bcrypt", req)


def get_html_msg(val: str, success: bool):
    invalid = not success
    return f"""<input readonly aria-invalid="{'true' if invalid else 'false'}" type="text" value="{val}"/>"""


@router.post("/bcrypt_hash")
async def bcrypt_hash_route(text: Annotated[str, Form()]):
    return bcrypt_hash(text)


@router.post("/bcrypt_salt")
async def bcrypt_salt_route():
    return bcrypt_salt()


@router.post("/bcrypt_hash_verify")
def bcrypt_hash_verify_route(
    text: Annotated[str, Form()], hash: Annotated[str, Form()]
):
    if len(hash) != 60:
        return get_html_msg("Incorrect Hash Length", False)
    verified = bcrypt_verify(text, hash)
    return get_html_msg("Valid Hash" if verified else "Invalid Hash", verified)


app_router.include_router(router)
