from pydantic import BaseModel
from pkg.config import UID


class PlayerModel(BaseModel):
    uid: int = UID
    nickname: str = "Ayaka"
    level: int = 70
    exp: int = 0
    stamina: int = 300
    mcoin: int = 4
    hcoin: int = 4
    scoin: int = 4
    world_level: int = 6
