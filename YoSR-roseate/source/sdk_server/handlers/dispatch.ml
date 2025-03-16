module Tiny = Tiny_httpd
open Protocol


let query_dispatch _ =
  let server_data = 
    default_server_data
      ~name:"roseate"
      ~display_name:"roseate"
      ~dispatch_url:Config.dispatch_url
      ~env_type:"2"
      ~title:"roseate"
      ~msg:"OK"
    () in

  let global_dispatch_data = 
    default_global_dispatch_data
      ~msg:"OK"
      ~server_list:[server_data]
    () in

  let encoder = Pbrt.Encoder.create () in
  encode_pb_global_dispatch_data global_dispatch_data encoder;

  let serialized_data = Pbrt.Encoder.to_string encoder in
  let base64_encoded_data = Base64.encode_string serialized_data in
  Tiny.Response.make_string (Ok base64_encoded_data)


let query_gateway _ =
  let gateserver = 
    default_gateserver
      ~ip:Config.game_server_host
      ~port:Config.game_server_port
      ~asset_bundle_url:Config.asset_bundle_url
      ~ex_resource_url:Config.ex_resource_url
      ~lua_url:Config.lua_url
      ~lua_version:Config.lua_version
      ~use_tcp:true
      ~unk1:true
      ~unk2:true
      ~unk3:true
      ~unk4:true
      ~unk5:true
      ~unk6:true
      ~unk7:true
    () in

  let encoder = Pbrt.Encoder.create () in
  encode_pb_gateserver gateserver encoder;

  let serialized_data = Pbrt.Encoder.to_string encoder in
  let base64_encoded_data = Base64.encode_string serialized_data in
  Tiny.Response.make_string (Ok base64_encoded_data)
