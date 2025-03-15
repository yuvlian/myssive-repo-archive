import asyncio
from nazo_rand import randrange
from pkg.game_server.net.kcp import get_conv
from pkg.game_server.net.packet import NetOperation
from pkg.game_server.net.session import PlayerSession
from pkg.game_server.logger import init_logging
from pkg.db.lib import get_database

logging = init_logging()


class KCPGateway(asyncio.DatagramProtocol):
    def __init__(self, db):
        self.id_counter = 0
        self.sessions = {}
        self.running = True
        self.shutdown_event = asyncio.Event()
        self.db = db

    def connection_made(self, transport):
        self.transport = transport
        logging.info(f"Listening on {self.transport.get_extra_info('sockname')}")

    def datagram_received(self, data, addr):
        data_len = len(data)
        logging.info(f"Received {data_len} bytes from {addr}: {[b for b in data]}")

        if data_len == 20:
            self.process_net_operation(NetOperation.from_bytes(data), addr)
        elif data_len >= 28:
            self.process_kcp_payload(data, addr)
        else:
            logging.warning("Unknown data length received")

    def process_net_operation(self, op: NetOperation, addr):
        if (op.head, op.tail) == (0xFF, 0xFFFFFFFF):
            self.establish_kcp_session(op.data, addr)
        elif (op.head, op.tail) == (0x194, 0x19419494):
            self.drop_kcp_session(op.conv_id)
        else:
            logging.warning(f"Unknown magic pair: {op.head}-{op.tail}")

    def establish_kcp_session(self, data, addr):
        conv_id, session_token = self.next_conv_pair()
        session_id = conv_id << 32 | session_token

        logging.info(f"New connection: {addr} with conv_id: {conv_id}")

        self.sessions[conv_id] = PlayerSession(
            self.transport, session_id, addr, self.db
        )

        net_op = NetOperation(
            head=0x145,
            conv_id=conv_id,
            session_token=session_token,
            data=data,
            tail=0x14514545,
        ).to_bytes()

        self.transport.sendto(net_op, addr)

    def drop_kcp_session(self, conv_id):
        if conv_id in self.sessions:
            del self.sessions[conv_id]
            logging.info(f"Dropped KCP session with conv_id: {conv_id}")
        else:
            logging.warning(
                f"Attempted to drop non-existent KCP session with conv_id: {conv_id}"
            )

    def process_kcp_payload(self, data, addr):
        conv_id = get_conv(data)
        session = self.sessions.get(conv_id)

        if session:
            asyncio.create_task(session.consume(data))
        else:
            logging.warning(f"Received data for unknown session conv_id: {conv_id}")

    def next_conv_pair(self):
        self.id_counter += 1
        return self.id_counter, randrange(0, 0xFF)

    def connection_lost(self, exc):
        logging.info("UDP connection lost, shutting down...")
        self.running = False
        self.shutdown_event.set()

    def shutdown(self):
        logging.info("Shutting down server...")
        self.running = False
        self.shutdown_event.set()

    @staticmethod
    async def new(host, port):
        loop = asyncio.get_running_loop()
        db = get_database()

        transport, protocol = await loop.create_datagram_endpoint(
            lambda: KCPGateway(db), local_addr=(host, port)
        )

        try:
            await protocol.shutdown_event.wait()
        except asyncio.CancelledError:
            logging.info("Server tasks cancelled.")
        finally:
            transport.close()
            logging.info("Server stopped.")
