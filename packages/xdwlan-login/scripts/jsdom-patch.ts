import { type Plugin } from "esbuild";
import { readFile } from "fs/promises";
import path from "path";

// See: https://github.com/evanw/esbuild/issues/1311#issuecomment-2489238892
export const jsdomPatch: Plugin = {
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
