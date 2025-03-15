from pkg.rail_proto.lib import PlayerLoginFinishScRsp


async def handle(req, db):
    return PlayerLoginFinishScRsp()
