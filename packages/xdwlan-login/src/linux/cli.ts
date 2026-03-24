import { Command } from "commander";
import readline from "node:readline";
import { getConfig, type Config } from "@lib/config";

function resolveValue(
  cliValue: string | undefined,
  envValue: string | undefined,
  configValue: string,
) {
  if (cliValue !== undefined && cliValue !== "") {
    return cliValue;
  }
  if (envValue !== undefined && envValue !== "") {
    return envValue;
  }
  return configValue;
}

interface Options extends Config {
  config?: string;
  oneshot: boolean;
}

export async function parseArgs(): Promise<Options> {
  const program = new Command();
  program
    .name("xdwlan-login")
    .description("Log into Xidian Campus Network.")
    .showHelpAfterError()
    .option("-c, --config <path>", "Path to config file")
    .option("-o, --oneshot", "Log in only once and exit", false)
    .option("-u, --username <username>", "Username for login")
    .option(
      "-p, --password <password>",
      "Password for login (leave empty to input interactively)",
    );
  program.parse();
  const parsed = program.opts<Partial<Options>>();

  const config = getConfig(parsed.config);
  const envUsername = process.env.XDWLAN_USERNAME;
  const envPassword = process.env.XDWLAN_PASSWORD;

  const options: Options = {
    ...config,
    oneshot: Boolean(parsed.oneshot),
    username: resolveValue(parsed.username, envUsername, config.username),
    password: resolveValue(parsed.password, envPassword, config.password),
  };

  if (!options.password) {
    const canReadStdin =
      !!process.stdin && process.stdin.readable && !process.stdin.destroyed;
    if (!canReadStdin) {
      console.error("Error: Password is required and stdin is not available.");
      process.exit(1);
    }

    if (process.stdin.isTTY) {
      process.stdout.write("Password: ");
    }

    const rl = readline.createInterface({ input: process.stdin });
    options.password = await new Promise((resolve) => {
      rl.on("line", (line) => {
        rl.close();
        if (process.stdin.isTTY) {
          process.stdout.write("\n");
        }
        resolve(line);
      });
    });

    if (!options.password) {
      console.error("Error: Password is required.");
      process.exit(1);
    }
  }

  return options;
}
