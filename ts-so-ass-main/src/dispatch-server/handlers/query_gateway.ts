import type { Context } from "hono";
import { GateServer } from "@scope/hsr-protobuf";
import { encodeBase64 } from "@std/encoding/base64";

export const getQueryGateway = (c: Context) => {
  const gateserver: GateServer = GateServer.create({
    ip: "127.0.0.1",
    port: 23301,
    useTcp: true,

    luaBundleVersionUpdateUrl: "",
    luaPatchVersion: "",
    designDataBundleVersionUpdateUrl: "",
    videoBundleVersionUpdateUrl: "",

    retcode: 0,
    watermarkEnable: false,
    enableVideoBundleVersionUpdate: true,
    closeRedeemCode: true,
    forbidRecharge: true,
    enableDesignDataBundleVersionUpdate: true,
    networkDiagnostic: false,
    androidMiddlePackageEnable: false,
    eventTrackingOpen: false,
    enableSaveReplayFile: true,
    enableUploadBattleLog: true,
    ejcaokobhbg: true,
    nhehajgmjnj: false,
    iosExam: false,
    mtpSwitch: false,
  });

  const serialized = GateServer.encode(gateserver).finish();
  const b64String = encodeBase64(serialized);

  return c.text(b64String);
};
