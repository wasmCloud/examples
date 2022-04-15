require('dotenv').config();
const path = require('path');

// craco.config.js
module.exports = {
  webpack: {
    configure: (webpackConfig, { env, paths }) => {
      webpackConfig.module.rules.push({
        test: /\.m?js/,
        resolve: {
          fullySpecified: false
        }
      })
      paths.appBuild = webpackConfig.output.path = path.resolve(__dirname, "dist")
      return webpackConfig;
    },
  }
}