# AyakaRail

Shitty & nonscalable private server for a certain chinese gacha game.

### requirements
- uv
- mongodb
- tls terminating proxy to redirect traffic

### tutorial
- clone this repo recursively and cd
- redirect game traffic to server's socketaddr with proxy
- `uv run -m pkg.game_server.main`
- `uv run -m pkg.http_server.main`

### license
- licensed under CC0-1.0 **EXCLUDING** [kcp.py](https://github.com/ayakaware/AyakaRail/blob/master/pkg/game_server/net/kcp.py) which is under GPL-3.0 from Mero <mero@crepe.moe>
