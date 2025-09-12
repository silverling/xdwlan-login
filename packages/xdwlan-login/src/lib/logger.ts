import pino from "pino";
import pretty from "pino-pretty";

export const validLevels = [
  "silent",
  "fatal",
  "error",
  "warn",
  "info",
  "debug",
  "trace",
];

// Read logging level from env XDWLAN_LOGIN_LOG_LEVEL, default to "info".
const level = () => {
  const logLevel = process.env.XDWLAN_LOGIN_LOG_LEVEL?.toLowerCase();
  if (logLevel && validLevels.includes(logLevel)) {
    return logLevel;
  }

  return "info";
};

const logPath = process.env.XDWLAN_LOGIN_LOG_PATH;
export const logger = pino(
  {
    level: level(),
  },
  pretty({
    colorize: !logPath,
    translateTime: "SYS:yyyy-mm-dd HH:MM:ss",
    ignore: "pid,hostname",
    destination: logPath ?? process.stdout,
    append: false,
  })
);
