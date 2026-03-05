import { Command } from "commander";
import readline from "node:readline";

export async function parseArgs() {
  interface Options {
    oneshot: boolean;
    username: string;
    password?: string;
  }

  const program = new Command();

  program
    .name("xdwlan-login")
    .description("Log into Xidian Campus Network.")
    .showHelpAfterError()
    .option("-o, --oneshot", "Log in only once and exit", false)
    .option("-u, --username <username>", "Username for login", process.env.XDWLAN_USERNAME)
    .option("-p, --password <password>", "Password for login (leave empty to input interactively)", process.env.XDWLAN_PASSWORD);

  program.parse();
  let options = program.opts<Options>();

  if (options.oneshot) {
    if (!options.username) {
      console.error("Error: Username is required in oneshot mode.");
      process.exit(1);
    }
    if (!options.password) {
      // Prompt for password if not provided, with input echoing suppressed
      process.stdout.write("Password: ");
      const rl = readline.createInterface({input: process.stdin});
      options.password = await new Promise((resolve) => {
        rl.on("line", (line) => {
          rl.close();
          process.stdout.write("\n");
          resolve(line);
        });
      });
    }
  }

  return options;
}
