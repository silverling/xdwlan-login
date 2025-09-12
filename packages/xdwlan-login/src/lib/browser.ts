import { JSDOM, VirtualConsole } from "jsdom";
import { logger } from "./logger";

export class Browser {
  private dom: JSDOM;
  private virtualConsole: VirtualConsole;

  constructor() {
    this.virtualConsole = new VirtualConsole();
    this.virtualConsole.on("jsdomError", (err) => {
      // @ts-ignore: the type is expected and should exists
      if (err.type === "not implemented") {
        // This message comes from patched version of jsdom.
        // We use it here to navigate manually.
        // See: https://github.com/jsdom/jsdom/issues/2112
        const newHref = (err.message as string).match(
          /navigate to ([^\s]*?)$/
        )?.[1];

        if (newHref) {
          logger.trace(`Browser: navigate to ${newHref}`);
          this.goto(newHref);
          return;
        }
      }
      logger.trace(`Browser: jsdomError: ${err}`);
    });

    this.dom = new JSDOM();
  }

  get window() {
    return this.dom.window;
  }

  get document() {
    return this.dom.window.document;
  }

  public async goto(url: string): Promise<void> {
    logger.trace(`Browser: visit ${url}`);

    // Every time goto is called, the DOM is replaced, so are the event listeners
    this.dom = await JSDOM.fromURL(url, {
      resources: "usable",
      runScripts: "dangerously",
      virtualConsole: this.virtualConsole,
    });

    // See: https://en.wikipedia.org/wiki/Meta_refresh#Examples
    const refreshTo = this.window.document.head
      .querySelector("meta[http-equiv='refresh']")
      ?.getAttribute("content")
      ?.split("url=", 2)
      .pop();
    if (refreshTo) {
      logger.trace(`Browser: meta refresh to ${refreshTo}`);
      // See: https://developer.mozilla.org/en-US/docs/Web/API/Location
      await this.goto(new URL(refreshTo, this.window.location.origin).href);
      return;
    }
  }
}
