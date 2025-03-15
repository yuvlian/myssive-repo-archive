import {
  PlayerHeartBeatCsReq,
  PlayerHeartBeatScRsp,
} from "@scope/hsr-protobuf";

export async function on_player_heart_beat_cs_req(p: Uint8Array): Uint8Array {
  const req = PlayerHeartBeatCsReq.decode(Reader.create(p));

  return PlayerHeartBeatScRsp.encode({
    clientTimeMs: req.clientTimeMs,
    serverTimeMs: req.clientTimeMs,
    retcode: 0,
  }).finish();
}

export async function on_get_basic_info_cs_req(_: Uint8Array): Uint8Array {
  return new Uint8Array();
}
