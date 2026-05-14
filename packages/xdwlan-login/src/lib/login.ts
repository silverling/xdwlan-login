import { sleep } from "bun";
import { Browser } from "./browser";
import { logger } from "./logger";
import { type Config } from "./config";

export async function login(config: Config) {
  logger.trace("LoginTask: start");

  const browser = new Browser();
  await browser.goto(config.url ?? "https://w.xidian.edu.cn/index_8.html");

  // Wait 5 seconds to make sure the page is fully loaded and navigation/redirections are complete.
  await sleep(5000);

  if (browser.window.location.pathname === "/srun_portal_success") {
    logger.trace("LoginTask: landed on /srun_portal_success");
    return;
  }

  if (browser.window.location.pathname === "/srun_portal_pc") {
    logger.trace("LoginTask: landed on /srun_portal_pc");

    // prettier-ignore
    {
      browser.document.querySelector<HTMLInputElement>("#username")!.value = config.username;
      browser.document.querySelector<HTMLInputElement>("#password")!.value = config.password;
      browser.document.querySelector<HTMLSelectElement>("#domain")!.value = config.domain;
      browser.document.querySelector<HTMLButtonElement>("#login-account")!.click();
    }

    logger.trace(
      `LoginTask: filled up login form, username: ${config.username} password: <redacted> domain: ${config.domain}`,
    );
    await sleep(5000); // Wait 5 seconds before check login status.
  } else {
    logger.info(`Unknown page: ${browser.window.location.href}`);
    return;
  }
}

export async function isOnline() {
  try {
    const controller = new AbortController();
    const signal = controller.signal;
    const abortRequest = setTimeout(() => {
      controller.abort();
      logger.trace("OnlineCheck: timeout, abort.");
    }, 5000); // timeout after 5 seconds

    logger.trace("OnlineCheck: start");
    const response = await fetch("http://www.baidu.com/", {
      credentials: "include",
      headers: {
        "User-Agent":
          "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:141.0) Gecko/20100101 Firefox/141.0",
        Accept:
          "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8",
        "Accept-Language": "zh-CN,en-US;q=0.7,en;q=0.3",
        "Sec-GPC": "1",
        "Upgrade-Insecure-Requests": "1",
        "Sec-Fetch-Dest": "document",
        "Sec-Fetch-Mode": "navigate",
        "Sec-Fetch-Site": "cross-site",
        Priority: "u=0, i",
      },
      method: "GET",
      mode: "cors",
      cache: "no-cache",
      keepalive: false,
      signal,
    }).then((resp) => {
      clearTimeout(abortRequest);
      return resp;
    });

    if (response.ok) {
      const text = await response.text();

      const isBaidu = text.includes("百度一下");
      if (isBaidu) return true;
      logger.trace(`"OnlineCheck: response text: ${text}"`);
    }

    logger.trace(`"OnlineCheck: response status: ${response.status}"`);
  } catch (err) {
    logger.trace(`"OnlineCheck: error: ${err}"`);
  }

  return false;
}
