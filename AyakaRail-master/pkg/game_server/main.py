import asyncio
from pkg.game_server.net.gateway import KCPGateway
from pkg.config import GAME_SERVER_HOST, GAME_SERVER_PORT


def fn_main():
    try:
        asyncio.run(KCPGateway.new(GAME_SERVER_HOST, GAME_SERVER_PORT))
    except Exception as e:
        print(f"Error: {e}")


fn_main()
