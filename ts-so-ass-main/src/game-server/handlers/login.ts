import {
  PlayerBasicInfo,
  PlayerGetTokenScRsp,
  PlayerLoginFinishScRsp,
  PlayerLoginScRsp,
} from "@scope/hsr-protobuf";

export async function on_player_get_token_cs_req(_: Uint8Array): Uint8Array {
  return PlayerGetTokenScRsp.encode({
    msg: "OK",
    retcode: 0,
    uid: 1,
  }).finish();
}

export async function on_player_login_cs_req(_: Uint8Array): Uint8Array {
  return PlayerLoginScRsp.encode({
    basicInfo: PlayerBasicInfo.create({
      nickname: "swag",
      level: 10,
      exp: 0,
      stamina: 240,
      mcoin: 1,
      hcoin: 1,
      scoin: 1,
      worldLevel: 1,
    }),
    retcode: 0,
    stamina: 240,
  }).finish();
}

export async function on_player_login_finish_cs_req(_: Uint8Array): Uint8Array {
  return PlayerLoginFinishScRsp.encode({
    retcode: 0,
  }).finish();
}
