import { decodePacket } from "../packet/decode.ts";
import { kingVon } from "./handler.ts";

// retard code
export async function handleConnection(conn: Deno.TcpConn) {
  let readFull = false;
  let fullBuffer: Uint8Array = new Uint8Array(2048);
  let tempBuffer: Uint8Array = new Uint8Array(1024);

  const tailMagic = new Uint8Array([0xD7, 0xA1, 0x52, 0xC8]);

  while (true) {
    const bytesRead = await conn.read(tempBuffer);

    if (bytesRead === null) {
      console.log("Connection closed");
      conn.close();
      break;
    }

    const data = tempBuffer.slice(0, bytesRead);

    const tailMagicIndex = data.lastIndexOf(tailMagic[0]);

    if (
      tailMagicIndex !== -1 &&
      data.slice(tailMagicIndex).every((v, i) => v === tailMagic[i])
    ) {
      if (!readFull) {
        const packet = decodePacket(data);
        const rsp = await kingVon(packet);
        await conn.write(rsp);
        readFull = false;
        fullBuffer = new Uint8Array(2048);
      }
    } else {
      fullBuffer = new Uint8Array([...fullBuffer, ...data]);
      readFull = true;
    }

    if (readFull && fullBuffer.length > 0) {
      const tailMagicIndex = fullBuffer.lastIndexOf(tailMagic[0]);

      if (
        tailMagicIndex !== -1 &&
        fullBuffer.slice(tailMagicIndex).every((v, i) => v === tailMagic[i])
      ) {
        const packet = decodePacket(fullBuffer);
        const rsp = await kingVon(packet);
        await conn.write(rsp);
        readFull = false;
        fullBuffer = new Uint8Array(2048);
      }
    }

    tempBuffer = new Uint8Array(1024);
  }
}
