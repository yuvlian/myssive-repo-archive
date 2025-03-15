import { encodePacket } from "../packet/encode.ts";
import type { SrPacket } from "../packet/type.ts";

// snake_case due to rust habits, cba to change to camelcase
import {
  on_player_get_token_cs_req,
  on_player_login_cs_req,
  on_player_login_finish_cs_req,
} from "../handlers/login.ts";
import {
  on_get_basic_info_cs_req,
  on_player_heart_beat_cs_req,
} from "../handlers/player.ts";

// king von 2012
export async function kingVon(packet: SrPacket): Promise<Uint8Array> {
  const req = packet.cmd;
  const body = packet.body;

  console.log(`got cmd ${req}`);

  // all of these handlers dont need async but maybe someone finna add database shit
  switch (req) {
    case 39: {
      const cmdid = 53;
      const rsp = await on_player_get_token_cs_req(body);
      return encodePacket(cmdid, rsp);
    }
    case 59: {
      const cmdid = 20;
      const rsp = await on_player_login_cs_req(body);
      return encodePacket(cmdid, rsp);
    }
    case 55: {
      const cmdid = 6;
      const rsp = await on_player_login_finish_cs_req(body);
      return encodePacket(cmdid, rsp);
    }
    case 96: {
      const cmdid = 31;
      const rsp = await on_player_heart_beat_cs_req(body);
      return encodePacket(cmdid, rsp);
    }
    case 2: {
      const cmdid = 66;
      const rsp = await on_get_basic_info_cs_req(body);
      return encodePacket(cmdid, rsp);
    }
    default: {
      return new Uint8Array();
    }
  }
}
