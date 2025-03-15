import type { Context } from "hono";

export const postAccountRiskyApiCheck = (c: Context) => {
  const rsp = {
    data: {},
    message: "OK",
    retcode: 0,
  };

  return c.json(rsp);
};
