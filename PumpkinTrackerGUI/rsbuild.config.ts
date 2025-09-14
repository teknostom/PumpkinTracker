import { defineConfig } from "@rsbuild/core";
import { pluginReact } from "@rsbuild/plugin-react";

export default defineConfig({
	plugins: [pluginReact()],
	output: {
		cssModules: {
			auto: true,
			localIdentName: "[name]__[local]___[hash:base64:5]",
		},
	},
});
