from fastapi.templating import Jinja2Templates
from starlette.types import Receive, Scope, Send
from fastapi.responses import FileResponse, RedirectResponse
from fastapi.staticfiles import StaticFiles
import os
from fastapi import Request
from typing import Dict, Optional


class sthaitik_sanchit(StaticFiles):
    """
    `स्थैतिकसञ्चितप्रदत्ता` : Static File Router, with `Cloudfare` like HTML Serving
    """

    async def __call__(self, scope: Scope, receive: Receive, send: Send) -> None:
        """
        The ASGI entry point.
        """
        assert scope["type"] == "http"

        if not self.config_checked:
            await self.check_config()
            self.config_checked = True

        path = scope["path"]
        nm = path.split("/")[-1]
        pth = f"{self.directory}/{path}"
        if path == "/":
            response = FileResponse(f"{self.directory}/index.html")
        elif path == "/index.html":
            response = RedirectResponse("/")
        elif os.path.isfile(f"{pth}.html"):
            response = FileResponse(f"{pth}.html")
        elif path.split(".")[-1] == "html" and os.path.isfile(pth):
            response = RedirectResponse(path[:-5])
        elif nm == "" and path[-1] == "/" and os.path.isfile(f"{pth[:-1]}.html"):
            response = RedirectResponse("/".join(path.split("/")[:-1]))
        else:
            self.html = True
            response = await self.get_response(self.get_path(scope), scope)
        await response(scope, receive, send)


JINJA_TEMPLATES = Jinja2Templates(directory="./_templ")


def render_page(name: str, request: Request, value: Optional[Dict] = None):
    if not value:
        value = {}
    value["request"] = request
    return JINJA_TEMPLATES.TemplateResponse(f"{name}.html", value)


# Make a copy of above with full type safety in future
