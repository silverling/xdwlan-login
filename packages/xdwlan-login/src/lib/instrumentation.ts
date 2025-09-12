import { logger } from "./logger";

export function exit() {
  logger.info("Exit.");
  setTimeout(() => {
    process.exit();
  }, 100);
}

export function handleSignal() {
  // Handle SIGINT
  process.on("SIGINT", exit);

  // Handle ctrl-c, the type of key is string because we set 'utf8' encoding.
  if (process.stdin.isTTY) {
    process.stdin.setRawMode(true); // Turn off echo
    process.stdin.resume();
    process.stdin.setEncoding("utf8");
    process.stdin.on("data", (key: string) => {
      if (key === "\u0003") {
        exit();
      }
    });
  }
}
