import { build, type BuildOptions } from "esbuild";
import { jsdomPatch } from "./jsdom-patch";

await build({
  entryPoints: [
    {
      in: "src/linux/main.ts",
      out: "xdwlan-login-linux",
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
  entrypoints: ["./build/xdwlan-login-linux.js"],
  outdir: "./build",
  compile: {
    target: "bun-linux-x64-modern-glibc",
    outfile: "xdwlan-login-linux",
  },
});
