import { Hono } from "hono";
import { logger } from "hono/logger";
import { onNotFound } from "./handlers/not_found.ts";
import { reverseProxy } from "./middleware/reverse_proxy.ts";

import { postProductComboGranterLoginV2Login } from "./handlers/product_combo_granter_login_v2_login.ts";
import { postProductMdkShieldApiLogin } from "./handlers/product_mdk_shield_api_login.ts";
import { postProductMdkShieldApiVerify } from "./handlers/product_mdk_shield_api_verify.ts";
import { postAccountRiskyApiCheck } from "./handlers/account_risky_api_check.ts";

const app = new Hono();

app.use(logger());
app.notFound(onNotFound);

// :xdskull:
app.get("/query_dispatch", reverseProxy);
app.get("/query_gateway", reverseProxy);

app.post(
  "/*/combo/granter/login/v2/login",
  postProductComboGranterLoginV2Login,
);
app.post("/*/mdk/shield/api/login", postProductMdkShieldApiLogin);
app.post("/*/mdk/shield/api/verify", postProductMdkShieldApiVerify);
app.post("/account/risky/api/check", postAccountRiskyApiCheck);

Deno.serve({ port: 21000 }, app.fetch);
