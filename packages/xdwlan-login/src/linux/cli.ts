import { Command } from "commander";

export function parseArgs() {
  interface Options {
    oneshot: boolean;
    username: string;
  }

  const program = new Command();

  program
    .name("xdwlan-login")
    .description("Log into Xidian Campus Network.")
    .showHelpAfterError()
    .option("-o, --oneshot", "Log in only once and exit", false);

  program.parse();
  return program.opts<Options>();
}
