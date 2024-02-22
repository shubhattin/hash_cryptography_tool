#!/usr/bin/env python3

from fastapi import FastAPI, Request
from fastapi.responses import Response
from fastapi.staticfiles import StaticFiles
from brotli_asgi import BrotliMiddleware
from datetime import timedelta
from .kry.datt import DEV_ENV
from .app import app_router

APP_NAME = "Simple Rent Record Analyser"
if DEV_ENV:
    app = FastAPI(debug=True, title=APP_NAME)
else:
    app = FastAPI(openapi_url=None, redoc_url=None, title=APP_NAME)


CACHE_DURATION = int(timedelta(hours=1).total_seconds())

app.add_middleware(BrotliMiddleware)


@app.middleware("http")
async def middleware(req: Request, call_next):
    res: Response = await call_next(req)
    head = {"X-sraShTA": "bhagavatprasAdAt"}
    if req.method == "GET":
        head.update(
            {
                "X-Robots-Tag": "noindex",
                "X-Frame-Options": "deny",
                "Cache-Control": "No-Store" if DEV_ENV else "public, max-age=0",
                # Using the E-tag caching instead
            }
        )
    for x in head:
        res.headers[x] = head[x]
    for x in ["X-Powered-By"]:
        if x in res.headers:
            del res.headers[x]
    return res


app.include_router(app_router)

app.mount("/", StaticFiles(directory="_static"), name="static")

if DEV_ENV:
    import uvicorn

    if __name__ == "__main__":
        uvicorn.run("main:app", host="0.0.0.0", port=3030, reload=True)
