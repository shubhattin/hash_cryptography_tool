"""
Page :- /
"""
from fastapi import APIRouter, Request, Form
from fastapi.responses import PlainTextResponse
from kry.plugins import render_page
from ._router import app_router
from typing import Annotated, Literal
from kry.gupta import (
    sha3_256,
    sha3_512,
    sha_256,
    sha_512,
    salt,
    encrypt_text,
    decrypt_text,
    to_base64,
    from_base64
)

router = APIRouter(prefix="", default_response_class=PlainTextResponse)


@router.get("/")
async def home_page(req: Request):
    return render_page("home", req)


def get_html_msg(val: str, success: bool):
    invalid = not success
    return f"""<input readonly aria-invalid="{'true' if invalid else 'false'}" type="text" value="{val}"/>"""


@router.post("/hash")
async def hash_route(
    name: Annotated[Literal["SHA", "SHA3"], Form()],
    number: Annotated[Literal["256", "512"], Form()],
    text: Annotated[str, Form()],
):
    if name == "SHA3":
        if number == "256":
            return sha3_256(text)
        elif number == "512":
            return sha3_512(text)
    elif name == "SHA":
        if number == "256":
            return sha_256(text)
        elif number == "512":
            return sha_512(text)


@router.post("/pass_hash")
async def pass_hash_route(
    name: Annotated[Literal["SHA", "SHA3"], Form()],
    number: Annotated[Literal["256", "512"], Form()],
    text: Annotated[str, Form()],
):
    slt = salt()
    if name == "SHA3":
        if number == "256":
            return sha3_256(text + slt) + slt
        elif number == "512":
            return sha3_512(text + slt) + slt
    elif name == "SHA":
        if number == "256":
            return sha_256(text + slt) + slt
        elif number == "512":
            return sha_512(text + slt) + slt


@router.post("/pass_hash_verify")
async def pass_hash_verify_route(
    name: Annotated[Literal["SHA", "SHA3"], Form()],
    number: Annotated[Literal["256", "512"], Form()],
    text: Annotated[str, Form()],
    hash: Annotated[str, Form()],
):
    LENGTH = {"512": 128, "256": 64}
    hash_length = LENGTH[number]
    if len(hash) < hash_length + 1:  # adding 1 as atleast salt should be there
        return get_html_msg("Insufficient hash length", success=False)
    slt = hash[hash_length:]
    hsh = hash[:hash_length]

    verified = False

    if name == "SHA3":
        if number == "256":
            verified = sha3_256(text + slt) == hsh
        elif number == "512":
            verified = sha3_512(text + slt) == hsh
    elif name == "SHA":
        if number == "256":
            verified = sha_256(text + slt) == hsh
        elif number == "512":
            verified = sha_512(text + slt) == hsh
    return get_html_msg("Valid Hash" if verified else "Invalid Hash", verified)


@router.post("/salt")
def get_salt_route():
    return salt()


def get_formatted_html(val: str, error=False):
    if not error:
        return f"""<textarea readonly>{val}</textarea>"""
    else:
        return f"""<input type="text" aria-invalid="true" readonly value="{val}"/>"""


# Encrpt/Descrypt
@router.post("/encrypt_decrypt")
def encrypt_decrypt_route(
    option: Annotated[Literal["encrypt", "decrypt"], Form()],
    text: Annotated[str, Form()],
    key: Annotated[str, Form()],
):
    if option == "encrypt":
        return get_formatted_html(encrypt_text(text, key))
    elif option == "decrypt":
        try:
            return get_formatted_html(decrypt_text(text, key))
        except:
            return get_formatted_html("Wrong Key!", error=True)


@router.post("/base64")
def base64_route(
    option: Annotated[Literal["encode", "decode"], Form()], text: Annotated[str, Form()]
):
    if option == "encode":
        return get_formatted_html(to_base64(text))
    elif option == "decode":
        try:
            return get_formatted_html(from_base64(text))
        except:
            return get_formatted_html("Invalid Base64!", True)


app_router.include_router(router)
