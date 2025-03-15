import type { Context } from "hono";

export const postProductComboGranterLoginV2Login = (c: Context) => {
  const rsp = {
    data: {
      account_type: 1,
      combo_id: "1",
      combo_token: "SuperSecureToken",
      data: '{"guest":false}',
      heartbeat: false,
      open_id: "1",
    },
    message: "OK",
    retcode: 0,
  };

  return c.json(rsp);
};
