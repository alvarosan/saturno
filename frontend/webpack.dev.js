const path = require("path");
//const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const HtmlWebpackPlugin = require("html-webpack-plugin");

module.exports = {
    mode: "development",
    entry: "./src/index.jsx",
    output: {
        filename: "[name].bundle.js",
        path: path.resolve(__dirname, "dist")
    },
    resolve: {
        // Add '.ts' and '.tsx' as resolvable extensions.
        extensions: [".js", ".jsx", ".wasm"]
    },
    devServer: {
        historyApiFallback: true,
        stats: "minimal"
    },
    devtool: "cheap-module-eval-source-map",
    plugins: [
        new HtmlWebpackPlugin(),
//        new WasmPackPlugin({
//            crateDirectory: path.resolve(__dirname, "../rendering_wasm"),
//            extraArgs: "--no-typescript",
//            withTypeScript: false, // this is new
//            forceMode: "development"
//        }),

    ],
    module: {
        rules: [
            {
                test: /\.css$/,
                use: ["style-loader", "css-loader"]
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
                enforce: "pre",
                test: /\.js$/,
                loader: "source-map-loader"
            },
            {
                // Had to move back to js due to issues loading wasm
                // library
                test: /\.(js|jsx)$/,
                exclude: /node_modules/,
                use: ['babel-loader']
            },
            {
                test: /\.wasm$/,
                    type: "webassembly/experimental"
            }
        ]
    }
};
