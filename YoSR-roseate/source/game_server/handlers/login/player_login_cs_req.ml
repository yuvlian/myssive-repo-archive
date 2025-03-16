open Command
open Protocol
open Packet

let handle _ =
  let player =
    default_player_basic_info
      ~nickname:"Yulian"
      ~level:5l
      ~stamina:240l
    () in

  let rsp =
    default_player_login_sc_rsp
      ~stamina:240l
      ~basic_info:(Some player)
    () in

  pack
    cmd_player_login_sc_rsp
    encode_pb_player_login_sc_rsp
    rsp
