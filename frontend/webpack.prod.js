const path = require("path");
const MiniCssExtractPlugin = require("mini-css-extract-plugin");
const CleanWebpackPlugin = require("clean-webpack-plugin");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = {
  mode: "production",
    entry: "./src/index.jsx",
  output: {
    filename: `[name].[hash].js`,
    path: path.resolve(__dirname, "dist")
  },
    resolve: {
        // Add '.ts' and '.tsx' as resolvable extensions.
        extensions: [".js", ".jsx", ".wasm"]
    },
  module: {
    rules: [
            {
		test: /\.css$/,
		use: [
		  {
		    loader: MiniCssExtractPlugin.loader
		  },
		  "css-loader"
		]
            },
            {
                test: /\.ts(x?)$/,
                exclude: /node_modules/,
                use: [
                    {
                        loader: "ts-loader"
                    }
                ]
            },
            {
                // Had to move back to js due to issues loading wasm
                // library
                enforce: "pre",
                test: /\.(js|jsx)$/,
                exclude: /node_modules/,
                use: ['babel-loader']
            },
            {
                test: /\.wasm$/,
                type: "webassembly/experimental"
            }
    ]
  },
  plugins: [
    new HtmlWebpackPlugin(),
      // always deletes the dist folder first in each build run.
      //    new CleanWebpackPlugin(["dist"], {
      //        root: __dirname // "dist" will be appended to this absolute path. See https://github.com/johnagan/clean-webpack-plugin#options-and-defaults-optional
      //            }),
    // the plugin to extract our css into separate .css files
    new MiniCssExtractPlugin({
      filename: "[name].[contenthash].css"
    }),
    // Trigger rust's wasm-pack build
    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, "../rendering_wasm"),
      withTypeScript: true, // this is new
      forceMode: "development"
    }),
  ],
  devtool: "source-map" // supposedly the ideal type without bloating bundle size
};
