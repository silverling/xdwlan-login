import { sleep } from "bun";
import { isOnline, login } from "./login";
import { logger } from "./logger";
import { handleSignal } from "./instrumentation";

let lastStatus: "online" | "offline" | "unknown" = "unknown";

async function main() {
  logger.trace("Main: start");
  if (await isOnline()) {
    // Status: offline/unknown -> online
    if (lastStatus !== "online") {
      logger.info("You are online.");
    }
    waitNextNetworkCheck();
    return;
  }

  // Status: online/unknown -> offline
  if (lastStatus !== "offline") {
    logger.info("Offline detected. Try to login.");
  }
  lastStatus = "offline";

  await login();

  if (await isOnline()) {
    logger.info("Login successfully.");
    waitNextNetworkCheck();
    return;
  }

  while (true) {
    logger.info("Login failed. Retry in 3 seconds.");
    await waitNextLoginRetry(3000);
    if (await isOnline()) {
      logger.info("Login successfully.");
      waitNextNetworkCheck();
      break;
    }
  }
}

function waitNextNetworkCheck() {
  lastStatus = "online";
  setTimeout(main, 60_000);
}

async function waitNextLoginRetry(delay: number) {
  await sleep(delay);
  await login();
}

{
  handleSignal();

  logger.info("Start.");
  main();
}
