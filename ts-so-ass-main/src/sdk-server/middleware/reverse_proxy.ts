import type { Context } from "hono";

const dispatchServer = "http://127.0.0.1:21001";

// Not really a reverse proxy...
// export const reverseProxy = async (c: Context) => {
//   const path = c.req.path;

//   if (path === "/query_dispatch" || path === "/query_gateway") {
//     const target = dispatchServer + path;

//     const rsp = await fetch(target, {
//       method: c.req.method,
//       headers: c.req.headers,
//       body: c.req.body,
//     });

//     return new Response(rsp.body, rsp);
//   }

//   return c.notFound();
// };

export const reverseProxy = async (c: Context) => {
  const target = dispatchServer + c.req.path;

  const rsp = await fetch(target, {
    method: c.req.method,
    headers: c.req.headers,
    body: c.req.body,
  });

  return new Response(rsp.body, rsp);
};
