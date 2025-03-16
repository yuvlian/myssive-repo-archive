open Lwt
open Lwt.Syntax
open Printf
open Packet
open Handlers

module CmdMap = Map.Make(Int)


let handler_map =
  let open Command in

  CmdMap.(
    empty
    |> add cmd_player_get_token_cs_req
        Login.Player_get_token_cs_req.handle

    |> add cmd_player_login_cs_req
        Login.Player_login_cs_req.handle

    |> add cmd_player_login_finish_cs_req
        Login.Player_login_finish_cs_req.handle

    |> add cmd_player_heart_beat_cs_req
        Player.Player_heart_beat_cs_req.handle

    |> add cmd_get_basic_info_cs_req
        Player.Get_basic_info_cs_req.handle

    |> add cmd_get_avatar_data_cs_req
        Avatar.Get_avatar_data_cs_req.handle

    |> add cmd_get_multi_path_avatar_info_cs_req
        Avatar.Get_multi_path_avatar_info_cs_req.handle

    |> add cmd_get_bag_cs_req
        Item.Get_bag_cs_req.handle

    |> add cmd_get_mission_status_cs_req
        Mission.Get_mission_status_cs_req.handle

    |> add cmd_get_cur_lineup_data_cs_req
        Lineup.Get_cur_lineup_data_cs_req.handle

    |> add cmd_get_cur_scene_info_cs_req
        Scene.Get_cur_scene_info_cs_req.handle

    (*|> add cmd_start_cocoon_stage_cs_req
        Battle.start_cocoon_stage_cs_req.handle

    |> add cmd_pvebattle_result_cs_req
        Battle.pvebattle_result_cs_req.handle *)
  )


let len_check str = 
  if String.length str = 0 then "Unhandled request"
  else Hex.show (Hex.of_string str)


let handle pk =
  match CmdMap.find_opt pk.cmd handler_map with
  | Some handler ->
      let body = len_check pk.body in
      printf "INFO   roseate_ml::game_server: Response body: %s\n%!" body; 
      Some (handler pk)
  | _ ->
      printf "INFO   roseate_ml::game_server: Response body: Unhandled request\n%!";
      None


let rec run ic oc =
  let* pk = read ic in
  printf "INFO   roseate_ml::game_server: Request command id: %d\n%!" pk.cmd;

  let* () =
    match handle pk with
    | Some res -> Lwt_list.iter_s (fun pk -> write oc pk) res.packets
    | _ -> return_unit
  in

  run ic oc
