import asyncio
from pkg.game_server.net.kcp import Kcp
from pkg.game_server.net.packet import NetPacket
from pkg.game_server.logger import init_logging
from pkg.game_server.handlers import player_login_finish, player_get_token, player_login
from pkg.rail_proto.lib import *
from pkg.rail_proto.cmd import *

logging = init_logging()


class PlayerSession:
    CMD_HANDLERS = {}

    def register_handler(self):
        handlers = [
            (
                PLAYER_GET_TOKEN_CS_REQ,
                PLAYER_GET_TOKEN_SC_RSP,
                player_get_token.handle,
                PlayerGetTokenCsReq,
            ),
            (
                PLAYER_LOGIN_CS_REQ,
                PLAYER_LOGIN_SC_RSP,
                player_login.handle,
                PlayerLoginCsReq,
            ),
            (
                PLAYER_LOGIN_FINISH_CS_REQ,
                PLAYER_LOGIN_FINISH_SC_RSP,
                player_login_finish.handle,
                PlayerLoginFinishCsReq,
            ),
        ]

        for h in handlers:
            req_cmd, rsp_cmd, handler_fn, req_msg = h
            self.CMD_HANDLERS[req_cmd] = {
                "fn": handler_fn,
                "req_msg": req_msg,
                "rsp_cmd": rsp_cmd,
            }

    def __init__(self, transport, session_id, client_addr, db):
        self.register_handler()
        self.transport = transport
        self.session_id = session_id
        self.client_addr = client_addr
        self.kcp = Kcp(session_id, self.send_output)
        self.is_destroyed = False
        self.db = db

    def send_output(self, data):
        self.transport.sendto(data, self.client_addr)

    async def consume(self, data):
        self.kcp.input(data)
        self.kcp.update(asyncio.get_running_loop().time())

        while True:
            packet = self.kcp.recv()
            if packet is None:
                break
            await self.handle_packet(packet)

        self.kcp.update(asyncio.get_running_loop().time())

    async def handle_packet(self, packet):
        net_packet = NetPacket.from_bytes(packet)
        cmd_id = net_packet.cmd_type

        logging.info(f"Received cmd: {cmd_id}")

        if cmd_id == 0x194:
            self.is_destroyed = True
            return

        handler = self.CMD_HANDLERS.get(cmd_id)

        if handler:
            try:
                message = net_packet.to_message(handler["req_msg"]())
                result = await handler["fn"](message, self.db)
                response_packet = NetPacket.from_message(handler["rsp_cmd"], result)
                await self.send(response_packet)
            except Exception as e:
                logging.error(f"Error handling cmd {cmd_id}: {e}")
        else:
            logging.warning(f"Unhandled cmd_id: {cmd_id}")

    async def send(self, packet):
        self.kcp.send(packet.to_bytes())
        self.kcp.flush()
        self.kcp.update(asyncio.get_running_loop().time())
