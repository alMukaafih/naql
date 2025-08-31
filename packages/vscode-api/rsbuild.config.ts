import { defineConfig } from "@rsbuild/core";
import { pluginZipFiles } from "./.pluginZipFiles";
import plugin from "./plugin.json";

export default defineConfig({
	tools: {
		bundlerChain: (_chain, { CHAIN_ID }) => {},
		htmlPlugin: false,
	},

	source: {
		entry: {
			main: {
				import: "./src/lib/index.js",
				publicPath: `__cdvfile_files-external__/plugins/${plugin.id}/`,
			},
		},
	},

	output: {
		target: "web",
		filename: {
			js: (pathData) => {
				if (pathData.chunk?.name === "main") {
					return "[name].js";
				}

				return "[name].[contenthash:8].js";
			},
		},
		distPath: {
			js: "",
			jsAsync: "",
		},
	},

	plugins: [pluginZipFiles()],
});
