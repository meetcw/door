const path = require("path");
const webpack = require("webpack");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const MiniCssExtractPlugin = require("mini-css-extract-plugin");
const { CleanWebpackPlugin } = require("clean-webpack-plugin");
const CopyWebpackPlugin = require("copy-webpack-plugin");

var Config =
  process.env.NODE_ENV == "production"
    ? require("./production.config")
    : require("./development.config");

function registerPage(config, name) {
  const basePath = "./src";
  const entry = path.resolve(path.resolve(basePath, name), "index.js");
  const page = path.resolve(path.resolve(basePath, name), "index.html");
  config.entry[name] = entry;
  config.plugins.push(
    new HtmlWebpackPlugin({
      // favicon: path.resolve(basePath, "./assets/logo.png"),
      template: page,
      filename: Config.getPageFilename(name),
      chunksSortMode: "auto",
      chunks: [name],
    })
  );
}
var config = {
  mode: Config.mode,
  entry: {},
  output: {
    path: Config.outputPath,
    filename: "./assets/[name].js",
    publicPath: "/",
  },
  module: {
    rules: [
      {
        test: /\.css$/,
        use: [MiniCssExtractPlugin.loader, "css-loader"],
      },
      {
        test: /\.less$/,
        use: [MiniCssExtractPlugin.loader, "css-loader", "less-loader"],
      },
      {
        test: /\.(png|svg|jpg|gif|woff|woff2|svg|eot|ttf)$/,
        use: [
          {
            loader: "url-loader",
          },
        ],
      },
    ],
  },
  plugins: [
    new webpack.HotModuleReplacementPlugin(),
    new MiniCssExtractPlugin({
      filename: "./assets/[name].css",
    }),
    new CleanWebpackPlugin(),
    new CopyWebpackPlugin({
      patterns: [
        {
          from: "./static/layout/*",
          to: Config.outputPath + "/layout/[name].[ext]",
        },
        {
          from: "./static/assets/*",
          to: Config.outputPath + "/assets/[name].[ext]",
        },
        {
          from: "./static/*",
          to: Config.outputPath + "/[name].[ext]",
        },
        // {
        //   from: "./static/iconfont.*",
        //   to: Config.outputPath + "/assets/[name].[ext]",
        // },
      ],
    }),
  ],
  devServer: {
    contentBase: path.join(__dirname, "dist"),
    compress: true,
    port: 9000,
    hot: true,
  },
};

registerPage(config, "index");
registerPage(config, "tags");
registerPage(config, "archives");
registerPage(config, "post");

module.exports = config;
