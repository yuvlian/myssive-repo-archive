import type { SrPacket } from "./type.ts";

export function decodePacket(buffer: Uint8Array): SrPacket {
  if (buffer.length < 16) {
    console.log(buffer);
    throw new Error("Byte array is too short");
  }

  const headMagic = new DataView(buffer.buffer, buffer.byteOffset, 4).getUint32(
    0,
    false,
  );

  if (headMagic !== 0x9D74C714) {
    console.log(buffer);
    throw new Error("Invalid head magic value");
  }

  const cmd = new DataView(buffer.buffer, buffer.byteOffset + 4, 2).getUint16(
    0,
    false,
  );

  const headSize = new DataView(buffer.buffer, buffer.byteOffset + 6, 2)
    .getUint16(0, false);

  const bodySize = new DataView(buffer.buffer, buffer.byteOffset + 8, 4)
    .getUint32(0, false);

  const headStart = 12;
  const headEnd = headStart + headSize;

  if (headEnd > buffer.length) {
    console.log(buffer);
    throw new Error("Head data exceeds byte array length");
  }

  const _headData = buffer.subarray(headStart, headEnd);

  const bodyStart = headEnd;
  const bodyEnd = bodyStart + bodySize;

  if (bodyEnd + 4 > buffer.length) {
    console.log(buffer);
    throw new Error("Body data exceeds byte array length");
  }

  const bodyData = buffer.subarray(bodyStart, bodyEnd);

  const tailMagic = new DataView(buffer.buffer, buffer.byteOffset + bodyEnd, 4)
    .getUint32(0, false);

  if (tailMagic !== 0xD7A152C8) {
    console.log(buffer);
    throw new Error("Invalid tail magic value");
  }

  return { cmd, bodyData };
}
