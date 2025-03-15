import type { Context } from "hono";
import { Dispatch, RegionInfo } from "@scope/hsr-protobuf";
import { encodeBase64 } from "@std/encoding/base64";

export const getQueryDispatch = (c: Context) => {
  const region: RegionInfo = RegionInfo.create({
    name: "swag",
    displayName: "swag",
    title: "swag",
    envType: "2",
    msg: "OK",
    dispatchUrl: "http://127.0.0.1:21001/query_gateway",
  });

  const dispatch: Dispatch = Dispatch.create({
    retcode: 0,
    msg: "OK",
    regionList: [region],
    topSeverRegionName: "swag",
  });

  const serialized = Dispatch.encode(dispatch).finish();
  const b64String = encodeBase64(serialized);

  return c.text(b64String);
};
