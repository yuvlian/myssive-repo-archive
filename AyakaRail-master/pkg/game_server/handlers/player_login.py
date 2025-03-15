from pkg.rail_proto.lib import PlayerLoginScRsp, PlayerBasicInfo
from pkg.config import UID
from pkg.db.models.user import PlayerModel
from pkg.game_server.utils import cur_timestamp_ms


async def handle(req, db):
    players_collection = db["players"]
    player_data = await players_collection.find_one({"uid": UID})

    if not player_data:
        player_data = PlayerModel().dict()
        await players_collection.insert_one(player_data)

    player = PlayerBasicInfo(
        nickname=player_data["nickname"],
        level=player_data["level"],
        exp=player_data["exp"],
        stamina=player_data["stamina"],
        mcoin=player_data["mcoin"],
        hcoin=player_data["hcoin"],
        scoin=player_data["scoin"],
        world_level=player_data["world_level"],
    )

    return PlayerLoginScRsp(
        basic_info=player,
        server_timestamp_ms=cur_timestamp_ms(),
        stamina=player_data["stamina"],
        retcode=0,
    )
