open Command
open Packet
open Protocol

let _lua_base_64 = "LS0gVGhpcyBkb2Vzbid0IGRvIGFueXRoaW5nIGluIHByb2QsIHNpbmNlIHRoZSBwcm90byB0aGF0IGVuYWJsZXMgYmV0YSBoaW50IGlzIHJlbW92ZWQNCmxvY2FsIGJldGFIaW50VGV4dENvbXBvbmVudCA9IENTLlVuaXR5RW5naW5lLkdhbWVPYmplY3QuRmluZCgiVUlSb290L0Fib3ZlRGlhbG9nL0JldGFIaW50RGlhbG9nKENsb25lKSIpDQogIDpHZXRDb21wb25lbnRJbkNoaWxkcmVuKHR5cGVvZihDUy5SUEcuQ2xpZW50LkxvY2FsaXplZFRleHQpKQ0KYmV0YUhpbnRUZXh0Q29tcG9uZW50LnRleHQgPSAiIg0KDQotLSBWZXJzaW9uIHRleHQsIGVtcHR5IHN0cmluZyBzbyB3ZSBkb24ndCBoYXZlIGFueSB1aWQgc2hvd24NCmxvY2FsIHZlcnNpb25UZXh0Q29tcG9uZW50ID0gQ1MuVW5pdHlFbmdpbmUuR2FtZU9iamVjdC5GaW5kKCJWZXJzaW9uVGV4dCIpDQogIDpHZXRDb21wb25lbnRJbkNoaWxkcmVuKHR5cGVvZihDUy5SUEcuQ2xpZW50LkxvY2FsaXplZFRleHQpKQ0KdmVyc2lvblRleHRDb21wb25lbnQudGV4dCA9ICIi"


let handle pk =
  let cur_time = Int64.of_float (Unix.gettimeofday() *. 1000.0) in
  let dec = Pbrt.Decoder.of_string pk.body in
  let req = decode_pb_player_heart_beat_cs_req dec in

  let rsp = 
    default_player_heart_beat_sc_rsp
      ~client_time_ms:req.client_time_ms
      ~server_time_ms:cur_time
      (* ~download_data:(Some lua_data) *)
    () in

  pack
    cmd_player_heart_beat_sc_rsp
    encode_pb_player_heart_beat_sc_rsp
    rsp
