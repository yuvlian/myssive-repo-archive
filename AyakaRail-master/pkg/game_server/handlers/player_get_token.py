from pkg.rail_proto.lib import PlayerGetTokenScRsp
from pkg.config import UID


async def handle(req, db):
    return PlayerGetTokenScRsp(msg="OK", retcode=0, uid=UID)
