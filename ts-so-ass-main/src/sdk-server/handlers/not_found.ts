import type { Context } from "hono";

export const onNotFound = (c: Context) => {
  return c.text("Not Found", 404);
};
