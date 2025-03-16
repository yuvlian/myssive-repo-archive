open Command
open Protocol
open Packet

let handle _ =
  pack
    cmd_player_get_token_sc_rsp
    encode_pb_player_get_token_sc_rsp 
    (default_player_get_token_sc_rsp ~uid:1l ())
