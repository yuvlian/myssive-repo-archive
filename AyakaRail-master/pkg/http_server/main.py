from fastapi import FastAPI
from pkg.http_server.handlers import auth, dispatch
from pkg.config import HTTP_SERVER_HOST, HTTP_SERVER_PORT
import uvicorn


def fn_main():
    app = FastAPI()
    app.include_router(auth.r)
    app.include_router(dispatch.r)

    uvicorn.run(app, host=HTTP_SERVER_HOST, port=HTTP_SERVER_PORT)


fn_main()
