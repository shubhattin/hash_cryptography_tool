from fastapi import APIRouter
from fastapi.responses import HTMLResponse

app_router = APIRouter(prefix="", default_response_class=HTMLResponse)
