require('dotenv').config();
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
      return webpackConfig;
    },
  }
}