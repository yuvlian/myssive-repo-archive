open Command
open Protocol
open Packet

let handle _ =
  let argenti = 
    default_lineup_avatar
      ~id:1302l
      ~hp:10000l
      ~slot:0l
      ~avatar_type:Avatar_formal_type
    () in
  
  let lineup_info = 
    default_lineup_info
      ~name:"roseate"
      ~avatar_list:[argenti]
    () in
  
  let rsp =
    default_get_cur_lineup_data_sc_rsp
      ~lineup:(Some lineup_info)
    () in

  pack
    cmd_get_cur_lineup_data_sc_rsp
    encode_pb_get_cur_lineup_data_sc_rsp
    rsp
