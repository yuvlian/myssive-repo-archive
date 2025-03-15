import { handleConnection } from "./connection.ts";

export async function startServer(port: number) {
  const listener = Deno.listen({ port: port, transport: "tcp" });
  console.log("TCP server is running on port", port);

  for await (const conn of listener) {
    await handleConnection(conn);
  }
}
