import { logger } from "@lib/logger";
import { exit, handleSignal } from "@lib/instrumentation";
import { oneshot } from "@lib/mode";
import { getConfig } from "@lib/config";

function main() {
  if (!process.env.XDWLAN_LOGIN_SERVER_PORT) {
    throw new Error("XDWLAN_LOGIN_SERVER_PORT not specified.");
  }

  handleSignal();
  const config = getConfig();

  let routineHandler: NodeJS.Timeout;
  async function routine() {
    await oneshot(config);
    routineHandler = setTimeout(routine, 60_000);
  }
  routine();

  const server = Bun.serve({
    hostname: "127.0.0.1",
    port: process.env.XDWLAN_LOGIN_SERVER_PORT,
    routes: {
      "/health": new Response("ok"),
      "/quit": {
        POST: () => {
          logger.trace("Received quit signal, shutting down...");
          clearTimeout(routineHandler);
          setTimeout(exit, 500);
          return new Response("ok");
        },
      },
      "/login": {
        POST: async (request) => {
          const status = await oneshot(config);
          return new Response(status ? "ok" : "no");
        },
      },
    },
  });

  logger.debug(`Server started at http://${server.hostname}:${server.port}`);
}

main();
