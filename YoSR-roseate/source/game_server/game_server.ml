let () = 
  Gateway.listen 
  Config.game_server_host
  (Int32.to_int Config.game_server_port)
