from fastapi import APIRouter
from fastapi.responses import PlainTextResponse
from pkg.config import (
    GAME_SERVER_HOST,
    GAME_SERVER_PORT,
    LUA_URL,
    LUA_VERSION,
    EX_RESOURCE_URL,
    ASSET_BUNDLE_URL,
    HTTP_SERVER_HOST,
    HTTP_SERVER_PORT,
)
from pkg.rail_proto.lib import Gateserver, ServerData, GlobalDispatchData
from base64 import b64encode

r = APIRouter()


@r.get("/query_dispatch", response_class=PlainTextResponse)
async def on_query_dispatch():
    rsp = bytes(
        GlobalDispatchData(
            msg="OK",
            top_sever_region_name="kamisato",
            server_list=[
                ServerData(
                    name="kamisato",
                    display_name="kamisato",
                    title="kamisato",
                    env_type="2",
                    msg="OK",
                    dispatch_url=f"http://{HTTP_SERVER_HOST}:{HTTP_SERVER_PORT}/query_gateway",
                )
            ],
        )
    )

    return b64encode(rsp)


@r.get("/query_gateway", response_class=PlainTextResponse)
async def on_query_gateway():
    rsp = bytes(
        Gateserver(
            use_tcp=False,
            ip=GAME_SERVER_HOST,
            port=GAME_SERVER_PORT,
            lua_url=LUA_URL,
            lua_version=LUA_VERSION,
            ex_resource_url=EX_RESOURCE_URL,
            asset_bundle_url=ASSET_BUNDLE_URL,
            unk1=True,
            unk2=True,
            unk3=True,
            unk4=True,
            unk5=True,
            unk6=True,
            unk7=True,
            unk9=True,
            unk10=True,
            unk11=True,
            unk12=True,
            unk13=True,
            unk14=True,
            unk15=True,
        )
    )

    return b64encode(rsp)
