export function encodePacket(cmdId: number, data: Uint8Array): Uint8Array {
  const packetLen = 12 + data.length + 4;
  const buffer = new Uint8Array(packetLen);
  const view = new DataView(buffer.buffer);

  let offset = 0;

  view.setUint32(offset, 0x9D74C714, false);
  offset += 4;

  view.setUint16(offset, cmdId, false);
  offset += 2;

  view.setUint16(offset, 0, false);
  offset += 2;

  view.setUint32(offset, data.length, false);
  offset += 4;

  buffer.set(data, offset);
  offset += data.length;

  view.setUint32(offset, 0xD7A152C8, false);

  return buffer;
}
