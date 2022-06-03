// const { defineConfig } = require('@vue/cli-service')
// module.exports = defineConfig({
//   transpileDependencies: true
// })
module.exports = {
  configureWebpack: {
    devtool: 'source-map',
  },
  devServer:{
    port: 8280
  }
}