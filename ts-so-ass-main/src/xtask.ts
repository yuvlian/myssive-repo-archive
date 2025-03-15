const commands = [
  "deno run -A sdk-server/main.ts",
  "deno run -A dispatch-server/main.ts",
  "deno run -A game-server/main.ts",
];

const isWindows = Deno.build.os === "windows";

async function runCommand(command) {
  console.log(`Starting: ${command}`);
  const cmd = isWindows
    ? ["cmd", "/c", "start", "cmd", "/k", command]
    : ["x-terminal-emulator", "-e", command];

  try {
    const commandOptions = {
      cmd: cmd,
      stdout: "inherit",
      stderr: "inherit",
    };

    const process = new Deno.Command(commandOptions.cmd[0], {
      args: commandOptions.cmd.slice(1),
      stdout: commandOptions.stdout,
      stderr: commandOptions.stderr,
    });

    const status = await process.spawn().status;

    if (!status.success) {
      console.error(`Process for command failed: ${command}`);
    }
  } catch (error) {
    console.error(`Failed to start terminal for command: ${command}`);
    console.error(error.message);
  }
}

for (const command of commands) {
  await runCommand(command);
}
