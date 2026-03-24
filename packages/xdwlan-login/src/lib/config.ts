import path from "path";
import fs from "fs";
import { getExecDir } from "./utils";
import { logger, validLevels } from "./logger";
import { z, ZodError } from "zod";

const ConfigSchema = z.object({
  username: z.string("学号配置错误").min(1),
  password: z.string("密码配置错误").min(1),
  domain: z
    .enum(
      ["", "@lt", "@dx", "@yd"],
      '校园网套餐配置错误，可选项有 "@lt", "@dx", "@yd", 普通校园网可留空',
    )
    .nullish()
    .transform((x) => x ?? undefined)
    .default(""),
  level: z.string().default("info").nullish(),
  url: z.url().nullish(),
});

function getConfigPath() {
  // From environment variable
  let configPath = process.env.XDWLAN_LOGIN_CONFIG_PATH;
  if (configPath && fs.existsSync(configPath)) {
    return configPath;
  }

  // From execution directory
  const execDir = getExecDir();
  configPath = path.join(execDir, "config.yaml");
  if (fs.existsSync(configPath)) {
    return configPath;
  }

  // From current directory
  configPath = path.join(process.cwd(), "config.yaml");
  if (fs.existsSync(configPath)) {
    return configPath;
  }

  logger.fatal("No config file found");
  process.exit(1);
}

function getConfig() {
  const configPath = getConfigPath();
  try {
    logger.info(`Loading config from ${configPath}`);
    const configContent = fs.readFileSync(configPath, "utf-8");

    // @ts-ignore -- Bun types are outdated, PR is merge, waiting for new release
    const config = ConfigSchema.parse(Bun.YAML.parse(configContent));

    if (config.level && validLevels.includes(config.level)) {
      logger.level = config.level;
      logger.debug(`Set log level to ${config.level}`);
    }

    return config;
  } catch (e: any) {
    if (e instanceof ZodError) {
      for (const issue of e.issues) {
        logger.fatal(`Config Error: [${issue.path}] ${issue.message}`);
      }
    } else {
      logger.fatal(e.message);
    }

    process.exit(1);
  }
}

export type Config = z.infer<typeof ConfigSchema>;
export const config = getConfig();
