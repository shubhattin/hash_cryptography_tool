"""
Page :- /
"""
from fastapi import APIRouter, Request, Form
from fastapi.responses import PlainTextResponse
from kry.plugins import render_page
from ._router import app_router
from typing import Annotated
from kry.gupta import sha3_256, sha3_512, sha_256, sha_512

router = APIRouter(prefix="", default_response_class=PlainTextResponse)


@router.get("/")
async def home_page(req: Request):
    return render_page("home", req)


@router.post("/hash")
async def hash(
    name: Annotated[str, Form()],
    number: Annotated[str, Form()],
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
    return "Error"


app_router.include_router(router)
