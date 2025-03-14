const path = require("path");
const TsconfigPathsPlugin = require("tsconfig-paths-webpack-plugin");

module.exports = {
  stories: ["../src/**/*.stories.@(js|jsx|ts|tsx|mdx)"],
  addons: [
    "@storybook/addon-links",
    "@storybook/addon-essentials",
    "@storybook/addon-interactions",
    {
      name: "@storybook/addon-postcss",
      options: {
        postcssLoaderOptions: {
          postcssOptions: {
            config: path.resolve(__dirname, "../postcss.config.js"),
          },
        },
      },
    },
  ],
  framework: {
    name: "@storybook/react-webpack5",
    options: {}
  },
  webpackFinal: async (config) => {
    config.module.rules.push(
      {
        test: /\.(ts|tsx)$/,
        exclude: /node_modules/,
        use: [
          {
            loader: "babel-loader",
            options: {
              presets: [
                "@babel/preset-env",
                [
                  "@babel/preset-react",
                  {
                    runtime: "automatic",       
                    importSource: "@emotion/react"  
                  }
                ],
                "@babel/preset-typescript"
              ],
            },
          },
        ],
      },
      {
        test: /\.(js|jsx)$/,
        exclude: /node_modules/,
        use: {
          loader: "babel-loader",
          options: {
            presets: ["@babel/preset-env", "@babel/preset-react"],
          },
        },
      }
    );

    // Make sure your config.resolve object exists
    if (!config.resolve) {
      config.resolve = {};
    }

    // Initialize config.resolve.plugins if needed
    if (!config.resolve.plugins) {
      config.resolve.plugins = [];
    }

    // Add tsconfig-paths-webpack-plugin
    config.resolve.plugins.push(
      new TsconfigPathsPlugin({
        configFile: path.resolve(__dirname, "../tsconfig.json"),
      })
    );

    config.resolve.extensions.push(".ts", ".tsx", ".js", ".jsx");
    return config;
  },
};
