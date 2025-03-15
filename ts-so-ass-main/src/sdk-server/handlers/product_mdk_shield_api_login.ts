import type { Context } from "hono";

export const postProductMdkShieldApiLogin = (c: Context) => {
  const rsp = {
    data: {
      account: {
        area_code: "**",
        email: "print@hello.world",
        country: "ID",
        is_email_verify: "1",
        token: "SuperSecureToken",
        uid: "1",
      },
      device_grant_required: false,
      reactivate_required: false,
      realperson_required: false,
      safe_mobile_required: false,
    },
    message: "OK",
    retcode: 0,
  };

  return c.json(rsp);
};
