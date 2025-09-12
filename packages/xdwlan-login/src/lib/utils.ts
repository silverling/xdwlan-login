import path from "path";

/**
 * Get the directory of the executable.
 * @returns The directory of the executable (not the current working directory)
 */
export function getExecDir(): string {
  const execPath = process.execPath;

  if (
    (process.platform === "win32" && path.basename(execPath) === "bun.exe") ||
    (process.platform !== "win32" && path.basename(execPath) === "bun")
  ) {
    return import.meta.dir;
  }

  return path.dirname(execPath);
}
