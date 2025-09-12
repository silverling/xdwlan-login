import { logger } from "@lib/logger";
import { parseArgs } from "./cli";
import { daemon, oneshot } from "@lib/mode";
import { exit, handleSignal } from "@lib/instrumentation";

async function main() {
  handleSignal();

  const options = parseArgs();

  if (options.oneshot) {
    logger.info("Start. Running in oneshot mode.");
    await oneshot();
    exit();
  } else {
    logger.info("Start. Running in daemon mode.");
    await daemon();
  }
}

main();
