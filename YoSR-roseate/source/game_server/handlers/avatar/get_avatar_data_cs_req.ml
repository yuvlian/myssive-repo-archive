open Command
open Protocol
open Packet


let handle pk =
  let dec = Pbrt.Decoder.of_string pk.body in
  let req = decode_pb_get_avatar_data_cs_req dec in

  let argenti = 
    default_avatar
      ~base_avatar_id:1302l
      ~level:80l
      ~promotion:6l
      ~rank:6l
    () in

  let rsp =
    default_get_avatar_data_sc_rsp
      ~is_all:req.is_get_all
      ~avatar_list:[argenti]
    () in

  pack
    cmd_get_avatar_data_sc_rsp
    encode_pb_get_avatar_data_sc_rsp
    rsp
