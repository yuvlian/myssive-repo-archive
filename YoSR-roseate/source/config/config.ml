let sdk_server_host = "127.0.0.1"
let sdk_server_port = 21000

let game_server_host = "127.0.0.1"
let game_server_port = 23301l

let dispatch_url = 
  Printf.sprintf "http://%s:%d/query_gateway" 
    sdk_server_host sdk_server_port

let lua_url = ""
let lua_version = ""
let ex_resource_url = ""
let asset_bundle_url = ""
