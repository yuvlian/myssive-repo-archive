from dataclasses import dataclass
import struct

HEAD_MAGIC = 0x9D74C714
TAIL_MAGIC = 0xD7A152C8


@dataclass
class NetPacket:
    cmd_type: int
    head: bytes
    body: bytes

    def to_message(self, m) -> "NetPacket":
        return m.parse(self.body)

    @staticmethod
    def from_message(c, m) -> "NetPacket":
        return NetPacket(cmd_type=c, head=[], body=bytes(m))

    def to_bytes(self) -> bytes:
        # packet_length = 12 + len(self.head) + len(self.body) + 4
        b = bytearray()

        b.extend(struct.pack(">I", HEAD_MAGIC))
        b.extend(struct.pack(">H", self.cmd_type))
        b.extend(struct.pack(">H", len(self.head)))
        b.extend(struct.pack(">I", len(self.body)))
        b.extend(self.head)
        b.extend(self.body)
        b.extend(struct.pack(">I", TAIL_MAGIC))

        return bytes(b)

    @staticmethod
    def from_bytes(b: bytes) -> "NetPacket":
        if len(b) < 16:
            raise ValueError("len(b) < 16")

        head_magic = struct.unpack_from(">I", b, 0)[0]

        if head_magic != HEAD_MAGIC:
            raise ValueError("Invalid head magic")

        cmd_type = struct.unpack_from(">H", b, 4)[0]
        head_length = struct.unpack_from(">H", b, 6)[0]
        body_length = struct.unpack_from(">I", b, 8)[0]

        head_start = 12
        head_end = head_start + head_length

        if head_end > len(b):
            raise ValueError("Head data > packet length")

        head = b[head_start:head_end]

        body_start = head_end
        body_end = body_start + body_length

        if body_end + 4 > len(b):
            raise ValueError("Body data > packet length")

        body = b[body_start:body_end]

        tail_magic = struct.unpack_from(">I", b, body_end)[0]

        if tail_magic != TAIL_MAGIC:
            raise ValueError("Invalid tail magic")

        return NetPacket(cmd_type, head, body)


@dataclass
class NetOperation:
    head: int
    conv_id: int
    session_token: int
    data: int
    tail: int

    def to_bytes(self) -> bytes:
        return struct.pack(
            ">IIIII", self.head, self.conv_id, self.session_token, self.data, self.tail
        )

    @staticmethod
    def from_bytes(b: bytes) -> "NetOperation":
        if len(b) != 20:
            raise ValueError("len(b) != 20")

        head, conv_id, session_token, data, tail = struct.unpack(">IIIII", b)

        return NetOperation(head, conv_id, session_token, data, tail)
