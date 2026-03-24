import { sleep } from "bun";
import { logger } from "./logger";
import { isOnline, login } from "./login";
import type { Config } from "./config";

/**
 *  Login once (if failed, retry up to 5 times)
 * @returns true if online after login, false otherwise
 */
export async function oneshot(config: Config): Promise<boolean> {
  logger.info("Checking network status.");
  if (await isOnline()) {
    logger.info("Already online.");
    return true;
  }

  // Try to login in max 5 attempts
  logger.info("Offline detected. Try to login.");
  for (let i = 1; i <= 5; ++i) {
    try {
      await login(config);

      if (await isOnline()) {
        logger.info("Login successfully.");
        return true;
      }
    } catch (err) {
      logger.error(err, `Error occurred while logging in attempt ${i}.`);
    }

    logger.info(`Login failed. ${5 - i} attempts remaining.`);
    await sleep(1000);
  }

  return await isOnline();
}

/**
 * Daemon mode: check network status every 60 seconds, if offline, try to login until success.
 * Never returns.
 */
export async function daemon(config: Config): Promise<never> {
  while (true) {
    if (await isOnline()) {
      // Check network status every 60 seconds
      await sleep(60_000);
      continue;
    }

    logger.info("Offline detected. Try to login.");
    while (true) {
      try {
        await login(config);

        if (await isOnline()) {
          logger.info("Login successfully.");
          break;
        } else {
          logger.info("Login failed. Retry in 3 seconds.");
          await sleep(3000);
        }
      } catch (err) {
        if (err instanceof Error) {
          if (err.code === "ECONNREFUSED") {
            logger.error(err, "Connection refused. Retry in 10 seconds.");
            await sleep(10000);
          }
        }

        logger.error(err, `Unhandled error: ${err}. Retry in 10 seconds.`);
        await sleep(10000);
      }
    }
  }
}
