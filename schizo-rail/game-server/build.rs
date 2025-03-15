use sr_proto::MsgTrait;
use sr_proto::cmd::*;
use sr_proto::pb::*;
use std::fs;
use std::path::Path;

const HEAD_MAGIC_BYTES: [u8; 4] = [0x9D, 0x74, 0xC7, 0x14];
const TAIL_MAGIC_BYTES: [u8; 4] = [0xD7, 0xA1, 0x52, 0xC8];

// const FINISHED_MAIN_MISSIONS: [u32; 365] = [
//     1000101, 1000111, 1000112, 1000113, 1000114, 1000201, 1000202, 1000203, 1000204, 1000300,
//     1000301, 1000302, 1000303, 1000304, 1000400, 1000401, 1000402, 1000410, 1000500, 1000501,
//     1000502, 1000503, 1000504, 1000505, 1000510, 1000511, 1010001, 1010002, 1010101, 1010201,
//     1010202, 1010203, 1010204, 1010205, 1010206, 1010301, 1010302, 1010303, 1010401, 1010405,
//     1010402, 1010403, 1010500, 1010501, 1010502, 1010503, 1010601, 1010602, 1010700, 1010701,
//     1010801, 1010802, 1010901, 1010902, 1011001, 1011002, 1011003, 1011100, 1011101, 1011102,
//     1011103, 1011201, 1011202, 1011301, 1011400, 1011401, 1011402, 1011403, 1011501, 1011502,
//     1011503, 1020101, 1020201, 1020302, 1020301, 1020400, 1020401, 1020402, 1020403, 1020501,
//     1020601, 1020701, 1020702, 1020801, 1020901, 1021001, 1021101, 1021201, 1021301, 1021401,
//     1021501, 1021601, 1021702, 1021703, 1030101, 1030102, 1030201, 1030202, 1030301, 1030302,
//     1030303, 1030304, 1030401, 1030402, 1030403, 1030501, 1030601, 1030701, 1030702, 1030801,
//     2000001, 2000002, 2000003, 2000004, 2000100, 2000101, 2000131, 2000132, 2000133, 2000110,
//     2000111, 2000301, 2000103, 2000112, 2000108, 2000104, 2000102, 2000105, 2000106, 2000107,
//     2000313, 2000314, 2000109, 2000113, 2000116, 2000118, 2000119, 2000120, 2000122, 2000302,
//     2000303, 2000304, 2000305, 2000310, 2000311, 2000312, 2000320, 2000701, 2000702, 2000703,
//     2000704, 2000705, 2000706, 2000707, 2000801, 2000802, 2000803, 2000901, 2000902, 2000903,
//     2001001, 2001002, 2001003, 2010005, 2010301, 2010302, 2011103, 2011104, 2011409, 2010401,
//     2010402, 2010405, 2010502, 2010503, 2010701, 2010708, 2010709, 2010720, 2010730, 2010731,
//     2010732, 2010733, 2010734, 2010735, 2010904, 2011101, 2011102, 2011105, 2011301, 2011302,
//     2011303, 2011501, 2011502, 2010909, 2010910, 2011601, 2011701, 2011801, 2011901, 2011902,
//     2011903, 2011904, 2011905, 2011906, 2020301, 2020302, 2020304, 2020316, 2020317, 2020318,
//     2020319, 2020401, 2020402, 2020403, 2020404, 2020405, 2020406, 2020407, 2020303, 2020103,
//     2020104, 2020105, 2020106, 2020107, 2020108, 2020109, 2020110, 2020111, 2020201, 2020202,
//     2020203, 2020204, 2020205, 2000201, 2000202, 2000203, 2000204, 2000205, 2000206, 2000207,
//     2000208, 2000209, 2000211, 2000212, 2010201, 2010202, 2010203, 2010204, 2010205, 2010206,
//     2010500, 2010501, 2010705, 2010706, 2010901, 2010902, 2010903, 2010702, 2010703, 2011400,
//     2011401, 2011406, 2011402, 2011403, 2011404, 2011405, 2011407, 2011408, 2011410, 2011411,
//     2011412, 2011413, 2010905, 2010906, 2010907, 2010908, 2010911, 2010912, 2020305, 2020306,
//     2020309, 2020307, 2020308, 2020701, 2020702, 2020703, 2020313, 2020314, 2020315, 6020101,
//     6020201, 6020202, 2020501, 2020502, 2020503, 2020504, 2020505, 2020506, 2020507, 2020601,
//     2020602, 2020603, 2020604, 2020801, 2020802, 2020901, 2021001, 2021002, 2021009, 2021601,
//     2021602, 2021701, 2021702, 2021703, 2021704, 2021705, 2021801, 2021802, 2021803, 2030001,
//     2030002, 2030003, 2030101, 2030102, 2030201, 2030202, 2030203, 2030301, 2030302, 3000201,
//     3000202, 3000203, 3000211, 3000212, 3000213, 3000301, 3000302, 3000303, 3000522, 3000523,
//     3000524, 3000525, 3000526, 3000527, 3000601, 3000602, 3000603, 3000604, 3000701, 3000702,
//     3000703, 3000704, 3000705, 3000800, 3000801, 3000802, 3000803, 3010102, 3010103, 3010104,
//     3010105, 3010201, 3010202, 3010203, 3010204,
// ];

fn encode_pk(cmd_id: u16, data: Vec<u8>) -> Vec<u8> {
    let packet_len = 12 + data.len() + 4;
    let mut buffer = Vec::with_capacity(packet_len);

    buffer.extend_from_slice(&HEAD_MAGIC_BYTES);
    buffer.extend_from_slice(&cmd_id.to_be_bytes());
    buffer.extend_from_slice(&0u16.to_be_bytes());
    buffer.extend_from_slice(&(data.len() as u32).to_be_bytes());
    buffer.extend_from_slice(&data);
    buffer.extend_from_slice(&TAIL_MAGIC_BYTES);

    buffer
}

fn main() {
    let player_get_token_sc_rsp = encode_pk(
        PLAYER_GET_TOKEN_SC_RSP,
        PlayerGetTokenScRsp {
            msg: String::from("OK"),
            retcode: 0,
            uid: 1,
            ..Default::default()
        }
        .encode_to_vec(),
    );

    let player_login_sc_rsp = encode_pk(
        PLAYER_LOGIN_SC_RSP,
        PlayerLoginScRsp {
            basic_info: Some(PlayerBasicInfo {
                nickname: String::from("a"),
                level: 1,
                exp: 0,
                stamina: 0,
                mcoin: 1,
                hcoin: 1,
                scoin: 1,
                world_level: 1,
            }),
            stamina: 0,
            ..Default::default()
        }
        .encode_to_vec(),
    );

    let player_login_finish_sc_rsp = encode_pk(
        PLAYER_LOGIN_FINISH_SC_RSP,
        PlayerLoginFinishScRsp { retcode: 0 }.encode_to_vec(),
    );

    let get_avatar_data_sc_rsp = encode_pk(
        GET_AVATAR_DATA_SC_RSP,
        GetAvatarDataScRsp {
            is_get_all: true,
            avatar_list: vec![Avatar {
                promotion: 0,
                rank: 0,
                exp: 0,
                level: 1,
                base_avatar_id: 1001,
                ..Default::default()
            }],
            ..Default::default()
        }
        .encode_to_vec(),
    );

    let get_cur_lineup_data_sc_rsp = encode_pk(
        GET_CUR_LINEUP_DATA_SC_RSP,
        GetCurLineupDataScRsp {
            retcode: 0,
            lineup: Some(LineupInfo {
                name: String::from("a"),
                avatar_list: vec![LineupAvatar {
                    id: 1001,
                    hp: 1,
                    slot_type: 0,
                    satiety: 0,
                    sp: Some(AmountInfo {
                        cur_amount: 0,
                        max_amount: 1,
                    }),
                    avatar_type: AvatarType::AvatarFormalType.into(),
                }],
                plane_id: 20101,
                ..Default::default()
            }),
        }
        .encode_to_vec(),
    );

    let get_mission_status_sc_rsp = encode_pk(
        GET_MISSION_STATUS_SC_RSP,
        GetMissionStatusScRsp {
            // finished_main_mission_id_list: FINISHED_MAIN_MISSIONS.to_vec(),
            // sub_mission_status_list: FINISHED_MAIN_MISSIONS
            //     .iter()
            //     .map(|i| Mission {
            //         id: *i,
            //         progress: 1,
            //         status: MissionStatus::MissionFinish.into(),
            //     })
            //     .collect(),
            ..Default::default()
        }
        .encode_to_vec(),
    );

    let player_heart_beat_sc_rsp = encode_pk(
        PLAYER_HEART_BEAT_SC_RSP,
        PlayerHeartBeatScRsp {
            client_time_ms: 0,
            server_time_ms: 1,
            ..Default::default()
        }
        .encode_to_vec(),
    );

    let get_basic_info_sc_rsp = encode_pk(
        GET_BASIC_INFO_SC_RSP,
        GetBasicInfoScRsp {
            is_gender_set: true,
            ..Default::default()
        }
        .encode_to_vec(),
    );

    let get_cur_scene_info_sc_rsp = encode_pk(
        GET_CUR_SCENE_INFO_SC_RSP,
        GetCurSceneInfoScRsp {
            scene: Some(SceneInfo {
                plane_id: 20101,
                floor_id: 20101_001,
                entry_id: 20101_01,
                game_mode_type: 2,
                scene_group_list: vec![SceneGroupInfo {
                    state: 1,
                    group_id: 0,
                    entity_list: vec![SceneEntityInfo {
                        group_id: 0,
                        inst_id: 0,
                        entity_id: 0,
                        actor: Some(SceneActorInfo {
                            avatar_type: AvatarType::AvatarFormalType.into(),
                            base_avatar_id: 1001,
                            map_layer: 2,
                            uid: 1,
                        }),
                        motion: Some(MotionInfo {
                            pos: Some(Vector { x: 0, y: 0, z: 0 }),
                            rot: None,
                        }),
                        ..Default::default()
                    }],
                }],
                ..Default::default()
            }),
            ..Default::default()
        }
        .encode_to_vec(),
    );

    let get_scene_map_info_sc_rsp = encode_pk(
        GET_SCENE_MAP_INFO_SC_RSP,
        GetSceneMapInfoScRsp {
            map_info_list: vec![SceneMapInfo {
                entry_id: 20101_01,
                ..Default::default()
            }],
            ..Default::default()
        }
        .encode_to_vec(),
    );

    let get_multi_path_avatar_info_sc_rsp =
        encode_pk(GET_MULTI_PATH_AVATAR_INFO_SC_RSP, Vec::new());
    let get_bag_sc_rsp = encode_pk(GET_BAG_SC_RSP, Vec::new());

    let dest_path = Path::new("./out/").join("rsp.rs");
    let output = format!(
        r#"const PLAYER_GET_TOKEN: [u8; {}] = {:?};
const PLAYER_LOGIN: [u8; {}] = {:?};
const PLAYER_LOGIN_FINISH: [u8; {}] = {:?};
const GET_AVATAR_DATA: [u8; {}] = {:?};
const GET_CUR_LINEUP_DATA: [u8; {}] = {:?};
const GET_MISSION_STATUS: [u8; {}] = {:?};
const PLAYER_HEART_BEAT: [u8; {}] = {:?};
const GET_BASIC_INFO: [u8; {}] = {:?};
const GET_CUR_SCENE_INFO: [u8; {}] = {:?};
const GET_SCENE_MAP_INFO: [u8; {}] = {:?};
const GET_MULTI_PATH_AVATAR_INFO: [u8; {}] = {:?};
const GET_BAG: [u8; {}] = {:?};
"#,
        player_get_token_sc_rsp.len(),
        player_get_token_sc_rsp,
        player_login_sc_rsp.len(),
        player_login_sc_rsp,
        player_login_finish_sc_rsp.len(),
        player_login_finish_sc_rsp,
        get_avatar_data_sc_rsp.len(),
        get_avatar_data_sc_rsp,
        get_cur_lineup_data_sc_rsp.len(),
        get_cur_lineup_data_sc_rsp,
        get_mission_status_sc_rsp.len(),
        get_mission_status_sc_rsp,
        player_heart_beat_sc_rsp.len(),
        player_heart_beat_sc_rsp,
        get_basic_info_sc_rsp.len(),
        get_basic_info_sc_rsp,
        get_cur_scene_info_sc_rsp.len(),
        get_cur_scene_info_sc_rsp,
        get_scene_map_info_sc_rsp.len(),
        get_scene_map_info_sc_rsp,
        get_multi_path_avatar_info_sc_rsp.len(),
        get_multi_path_avatar_info_sc_rsp,
        get_bag_sc_rsp.len(),
        get_bag_sc_rsp
    );

    fs::write(dest_path, output).expect("Unable to write file");
}
