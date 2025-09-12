import { build, type BuildOptions } from "esbuild";
import { jsdomPatch } from "./jsdom-patch";
import path from "path";

await build({
  entryPoints: [
    {
      in: "src/windows/main.ts",
      out: "xdwlan-login-windows-server",
    },
  ],
  outdir: "build",
  bundle: true,
  platform: "node",
  format: "esm",
  external: ["bun"],
  minify: false,
  plugins: [jsdomPatch],
} satisfies BuildOptions);

await Bun.build({
  entrypoints: ["./build/xdwlan-login-windows-server.js"],
  outdir: "./build",
  sourcemap: true,
  compile: {
    target: "bun-windows-x64-modern",
    outfile: "xdwlan-login-windows-server",
    windows: {
      title: "Xidian WLAN Login Server",
      publisher: "Silver Ling",
      description: "A local server for Xidian WLAN auto login.",
      hideConsole: false, // This seems broken. https://github.com/oven-sh/bun/issues/19916
      icon: path.normalize(
        path.join(process.cwd(), "../../resources/icons/avocado.ico")
      ),
    },
  },
});
