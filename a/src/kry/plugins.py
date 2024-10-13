from fastapi.templating import Jinja2Templates
from fastapi import Request
from typing import Dict, Optional

JINJA_TEMPLATES = Jinja2Templates(directory="./_templ")


def render_page(name: str, request: Request, value: Optional[Dict] = None):
    if not value:
        value = {}
    value["request"] = request
    return JINJA_TEMPLATES.TemplateResponse(f"{name}.html", value)
