import { build, type BuildOptions, type Plugin } from "esbuild";
import { readFile } from "fs/promises";
import path from "path";

// See: https://github.com/evanw/esbuild/issues/1311#issuecomment-2489238892
const jsdomPatch: Plugin = {
  name: "jsdom-patch",
  setup(build) {
    build.onLoad({ filter: /XMLHttpRequest-impl\.js$/ }, async (args) => {
      let contents = await readFile(args.path, "utf8");
      contents = contents.replace(
        'const syncWorkerFile = require.resolve ? require.resolve("./xhr-sync-worker.js") : null;',
        `const syncWorkerFile = "${require
          .resolve("jsdom/lib/jsdom/living/xhr/xhr-sync-worker.js")
          .replaceAll("\\", "\\\\")}";`,
      );
      return {
        contents,
        loader: "js",
        resolveDir: path.dirname(args.path),
      };
    });
  },
};

async function main() {
  const args = process.argv.slice(2);
  if (args.length === 0) {
    console.error("Available targets: linux, windows.");
    process.exit(1);
  }
  const targets = args;
  const targetConfig = {
    linux: {
      in: "src/linux/main.ts",
      out: "xdwlan-login-linux",
    },
    windows: {
      in: "src/windows/main.ts",
      out: "xdwlan-login-windows-server",
    },
  };

  for (const target of targets) {
    if (!(target in targetConfig)) {
      console.error(`Unknown target: ${target}`);
      process.exit(1);
    }

    console.log(`Building for target: ${target}`);
    await build({
      entryPoints: [targetConfig[target as keyof typeof targetConfig]],
      outdir: "build",
      bundle: true,
      platform: "node",
      format: "esm",
      external: ["bun"],
      minify: false,
      plugins: [jsdomPatch],
    } satisfies BuildOptions);
  }
}

main();
