import { logger } from "@lib/logger";
import { parseArgs } from "./cli";
import { exit, handleSignal } from "@lib/instrumentation";

async function main() {
  handleSignal();
  const options = await parseArgs();

  if (options.oneshot) {
    const oneshot = await import("@lib/mode").then((m) => m.oneshot);
    logger.info("Start. Running in oneshot mode.");
    await oneshot(options);
    exit();
  } else {
    const daemon = await import("@lib/mode").then((m) => m.daemon);
    logger.info("Start. Running in daemon mode.");
    await daemon(options);
  }
}

main();
