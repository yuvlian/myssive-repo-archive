open Printf
module Tiny = Tiny_httpd
module Dispatch = Handlers.Dispatch
module Auth = Handlers.Auth


let () =
  let server = 
    Tiny.create 
      ~addr:Config.sdk_server_host 
      ~port:Config.sdk_server_port 
    () in

  Tiny.add_route_handler ~meth:`GET server
    Tiny.Route.(exact_path "query_dispatch" return)
    Dispatch.query_dispatch;

  Tiny.add_route_handler ~meth:`GET server
    Tiny.Route.(exact_path "query_gateway" return)
    Dispatch.query_gateway;

  Tiny.add_route_handler ~meth:`POST server
    Tiny.Route.(exact_path "account/risky/api/check" return)
    Auth.risky_api_check;

  Tiny.add_route_handler ~meth:`POST server
    Tiny.Route.(exact_path "hkrpg_global/mdk/shield/api/login" return)
    Auth.mdk_shield_login;

  Tiny.add_route_handler ~meth:`POST server
    Tiny.Route.(exact_path "hkrpg_global/mdk/shield/api/verify" return)
    Auth.mdk_shield_login;

  Tiny.add_route_handler ~meth:`POST server
    Tiny.Route.(exact_path "hkrpg_global/combo/granter/login/v2/login" return)
    Auth.granter_login_v2;

  printf 
    "INFO   roseate_ml::sdk_server: Listening at %s:%d\n%!"
    (Tiny.addr server) (Tiny.port server);

  match Tiny.run server with
    | Ok () -> ()
    | Error e -> raise e
