import { Hono } from "hono";
import { logger } from "hono/logger";

import { onNotFound } from "./handlers/not_found.ts";
import { getQueryDispatch } from "./handlers/query_dispatch.ts";
import { getQueryGateway } from "./handlers/query_gateway.ts";

const app = new Hono();

app.use(logger());
app.notFound(onNotFound);

app.get("/query_dispatch", getQueryDispatch);
app.get("/query_gateway", getQueryGateway);

Deno.serve({ port: 21001 }, app.fetch);
