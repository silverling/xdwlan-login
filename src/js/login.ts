import { Browser, type BrowserPage, type Element, type Location, type Response } from "happy-dom";
import process from "node:process";

const userAgent = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:137.0) Gecko/20100101 Firefox/137.0";

// A virtual browser that simulates fetch and DOM manipulation.
class VirtualBrowser {
  private browser: Browser;
  public page: BrowserPage;
  private scriptsBlackList: string[];

  constructor() {
    this.browser = new Browser({
      settings: {
        navigator: {
          userAgent: userAgent,
        },
        disableJavaScriptFileLoading: true,
        disableJavaScriptEvaluation: true,
        disableCSSFileLoading: true,
        disableComputedStyleRendering: true,
        handleDisabledFileLoadingAsSuccess: true,
      },
    });
    this.page = this.browser.newPage();
    this.scriptsBlackList = [];
  }

  /** Get the current location of the page.
   *
   * @returns Current location of the page.
   */
  public getCurrentLocation(): Location {
    return this.page.mainFrame.window.location;
  }

  /** Fetch script content from URL.
   *
   * @param url script URL
   * @returns
   */
  private async fetchScript(url: URL): Promise<string> {
    console.debug(`[virtual-browser] fetch ${url.href}`);
    const resp = await fetch(url, {
      mode: "no-cors",
      method: "GET",
      referrer: this.page.url,
      headers: {
        "User-Agent": userAgent,
        Accept: "*/*",
        "Accept-Language": "zh-CN,en-US;q=0.7,en;q=0.3",
        "Sec-GPC": "1",
        "Sec-Fetch-Dest": "script",
        "Sec-Fetch-Mode": "no-cors",
        "Sec-Fetch-Site": "same-origin",
      },
    });
    return await resp.text();
  }

  /** Evaluate the script in current page context.
   *
   * Happy-DOM's evaluation sometimes differs from what I expected.
   * So I set disableJavaScriptFileLoading and disableJavaScriptEvaluation to true, then evalute it by myself.
   *
   * @param element script element, either remote script or inline.
   */
  private async evaluateScriptTag(element: Element): Promise<void> {
    const src = element.getAttribute("src");
    if (src) {
      // remote script
      const url = new URL(
        `${this.page.mainFrame.window.location.origin}/${src}`,
      );
      for (const disabledScript of this.scriptsBlackList) {
        if (url.pathname.endsWith(disabledScript)) {
          console.debug(`[virtual-browser] skip ${url.href}`);
          return;
        }
      }

      const code = await this.fetchScript(url);
      console.debug(`[virtual-browser] evaluate ${url.href}`);
      this.page.evaluate(code);
    } else if (element.innerHTML) {
      // inline script
      console.debug("[virtual-browser] evaluate inline script");
      this.page.evaluate(element.innerHTML);
    }
  }

  /** This function extends happy-dom page routing with handler for meta refresh tag and evaluation of script tag.
   *
   * @param url URL to navigate to
   * @returns Response object or null if the navigation failed
   */
  async goto(url: string): Promise<Response | null> {
    console.debug(`[virtual-browser] goto ${url}`);
    const resp = await this.page.goto(url);
    await this.wait();

    // Handle meta refresh tag
    for (const element of this.page.mainFrame.document.head.children) {
      if (
        element.tagName === "META" &&
        element.getAttribute("http-equiv") === "refresh"
      ) {
        const metaContent = element.getAttribute("content");
        if (metaContent !== null) {
          const subUrl = metaContent.split(";url=", 2)[1];
          if (subUrl) {
            const url = `${this.page.mainFrame.document.location.origin}/${subUrl}`;
            return this.goto(url);
          }
        }
      }
    }

    // Evaluate script tag in HEAD and BODY
    for (const element of this.page.mainFrame.document.head.children) {
      if (element.tagName === "SCRIPT") {
        await this.evaluateScriptTag(element);
      }
    }
    for (const element of this.page.mainFrame.document.body.children) {
      if (element.tagName === "SCRIPT") {
        await this.evaluateScriptTag(element);
      }
    }

    await this.wait();

    return resp;
  }

  public async wait(): Promise<void> {
    await this.page.waitUntilComplete();
    console.debug(`[virtual-browser] current at ${this.getCurrentLocation().href}`);
  }
}

function checkEnvVars() {
  if (!process.env.XDWLAN_LOGIN_URL) {
    throw new Error("Environment variable XDWLAN_LOGIN_URL is not set");
  }
  if (!process.env.XDWLAN_USERNAME) {
    throw new Error("Environment variable XDWLAN_USERNAME is not set");
  }
  if (!process.env.XDWLAN_PASSWORD) {
    throw new Error("Environment variable XDWLAN_PASSWORD is not set");
  }
}

/** Simulate login to Xidian University WLAN.
 *
 * @returns Promise<void>
 */
async function main() {
  checkEnvVars();

  const browser = new VirtualBrowser();
  await browser.goto(process.env.XDWLAN_LOGIN_URL); // It is usually https://w.xidian.edu.cn/index_8.html

  // Check if we are already on the success page
  function isOnBoard(): boolean {
    return browser.getCurrentLocation().pathname === "/srun_portal_success";
  }

  // Automatically login
  if (isOnBoard()) {
    console.log("[virtual-browser] successfully auto logged in.");
    return;
  }

  // Manually login
  console.log("[virtual-browser] try to login.");
  browser.page.evaluate(
    `
		(function login() {
			window.xdwlan_login = { message: "not yet" };
			if (document.querySelector("div.control > button.btn-confirm")) {
				document.querySelector("div.control > button.btn-confirm").click();
			}
			document.querySelector("#username").value = "username_placeholder";
			document.querySelector("#password").value = "password_placeholder";
			document.querySelector("#domain").value = "domain_placeholder";
			document.querySelector("#login-account").click();
			window.xdwlan_login = { message: "ok" };
		})()
	`
      .replace("username_placeholder", process.env.XDWLAN_USERNAME)
      .replace("password_placeholder", process.env.XDWLAN_PASSWORD)
      .replace("domain_placeholder", process.env.XDWLAN_DOMAIN ?? ""),
  );
  await browser.wait();

  if (isOnBoard()) {
    console.log("[virtual-browser] successfully logged in.");
  }

  console.log("[virtual-browser] finished login task.");
}

main();
