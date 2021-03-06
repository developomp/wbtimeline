const path = require("path")
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin")
const CopyWebpackPlugin = require("copy-webpack-plugin")

const distPath = path.resolve(__dirname, "dist")
module.exports = (_, argv) => {
	return {
		experiments: {
			asyncWebAssembly: true,
		},
		devServer: {
			static: {
				directory: distPath,
			},
			compress: true,
			port: 8000,
		},
		entry: "./bootstrap.js",
		output: {
			path: distPath,
			filename: "main.js",
			webassemblyModuleFilename: "main.wasm",
		},
		module: {
			rules: [
				{
					test: /\.s[ac]ss$/i,
					use: ["style-loader", "css-loader", "sass-loader"],
				},
			],
		},
		plugins: [
			new CopyWebpackPlugin({
				patterns: [{ from: "./static", to: distPath }],
			}),
			new WasmPackPlugin({
				crateDirectory: ".",
				extraArgs: "--no-typescript",
			}),
		],
	}
}
