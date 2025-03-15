a login simulator for HSR made with Deno 2.0 to test:
- Deno quirks
- Deno monorepo
- Using protocol buffers & tcp server with deno

the last point being the most important because holy shit using protos in typescript is so autistic, just like my code

if you really want to use this abomination, uhh

`cd src`

`deno run xtask.ts`

note: i am serious when i say this is just a login simulator, you can easily add handlers though, if you really want to. look at `.\src\game-server\handlers\player.ts` for example of decoding request, sending something, and sending dummy.

as of 2.7.0, these are the required gameserver requests to be handled to get into the game:

- player get token
- player login
- player login finish
- player heartbeat
- get basic info (can be dummy)
- get avatar data
- get multi path avatar info (can be dummy)
- get bag (can be dummy)
- get mission status
- get cur lineup data
- get cur scene info
