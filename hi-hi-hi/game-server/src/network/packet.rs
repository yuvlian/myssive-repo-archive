use hi3_proto::Conv;

/// -> (CmdId, UserId, Body)
pub fn decode(buf: &[u8]) -> (u32, u32, &[u8]) {
    if buf.len() < 38 {
        panic!("buf too small")
    }

    let head_magic = u32::from_be_bytes(buf[0..4].try_into().expect("Invalid head magic slice"));

    if head_magic != 0x01234567 {
        panic!("invalid head magic")
    }

    let user_id = u32::from_be_bytes(buf[12..16].try_into().expect("invalid user_id slice"));

    let cmd_id = u32::from_be_bytes(buf[24..28].try_into().expect("invalid cmdid slice"));

    let header_len =
        u16::from_be_bytes(buf[28..30].try_into().expect("invalid header len slice")) as usize;

    let body_len =
        u32::from_be_bytes(buf[30..34].try_into().expect("invalid body len slice")) as usize;

    if buf.len() < 34 + header_len + body_len + 4 {
        panic!("buf too small for data")
    }

    let header_start = 34;
    let header_end = header_start + header_len;
    // let header = &buf[header_start..header_end];

    let body_start = header_end;
    let body_end = body_start + body_len as usize;
    let body = &buf[body_start..body_end];

    // We don't need to check tail magic since the connection loop does that already.

    (cmd_id, user_id, body)
}

fn create_packet(
    user_id: u32,
    cmd_id: u32,
    header_len: u16,
    body_len: u32,
    header: Vec<u8>,
    body: Vec<u8>,
) -> Vec<u8> {
    let pk_len = 34 // fixed size (without tail)
        + header_len as usize
        + body_len as usize
        + 4; // tail

    // allocate
    let mut pk = Vec::with_capacity(pk_len);

    pk.extend_from_slice(&0x01234567u32.to_be_bytes()); // head magic
    pk.extend_from_slice(&1u16.to_be_bytes()); // packet version
    pk.extend_from_slice(&0u16.to_be_bytes()); // client version
    pk.extend_from_slice(&0u32.to_be_bytes()); // sequence number
    pk.extend_from_slice(&user_id.to_be_bytes()); // user id
    pk.extend_from_slice(&0u32.to_be_bytes()); // reserved1
    pk.extend_from_slice(&0u32.to_be_bytes()); // reserved2
    pk.extend_from_slice(&cmd_id.to_be_bytes()); // cmd id
    pk.extend_from_slice(&header_len.to_be_bytes()); // header length
    pk.extend_from_slice(&body_len.to_be_bytes()); // body length
    pk.extend_from_slice(&header); // header data
    pk.extend_from_slice(&body); // body data
    pk.extend_from_slice(&0x89ABCDEFu32.to_be_bytes()); // tail magic

    pk
}

// pub fn full_data(user_id: u32, cmd_id: u32, header: Vec<u8>, body: Vec<u8>) -> Vec<u8> {
//     let header_len = header.len() as u16;
//     let body_len = body.len() as u32;
//     create_packet(cmd_id, user_id, header_len, body_len, header, body)
// }

pub fn some_data<T: Conv>(user_id: u32, cmd_id: u32, body: T) -> Vec<u8> {
    let body = body.encode_to_vec();
    let body_len = body.len() as u32;
    create_packet(user_id, cmd_id, 0u16, body_len, Vec::new(), body)
}

// dummy
#[allow(dead_code)]
pub fn no_data(user_id: u32, cmd_id: u32) -> Vec<u8> {
    create_packet(user_id, cmd_id, 0u16, 0u32, Vec::new(), Vec::new())
}
