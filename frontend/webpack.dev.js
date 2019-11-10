const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");

module.exports = {
    mode: "development",
    entry: "./src/index.js",
    output: {
        filename: "[name].bundle.js",
        path: path.resolve(__dirname, "dist")
    },
    devServer: {
        historyApiFallback: true,
        stats: "minimal"
    },
    devtool: "cheap-module-eval-source-map",
    plugins: [new HtmlWebpackPlugin()],
    module: {
        rules: [
            {
                test: /\.css$/,
                use: ["style-loader", "css-loader"]
            },
            {
                test: /\.js$/,
                exclude: /node_modules/,
                use: 'babel-loader'
            }
        ]
    }
};
