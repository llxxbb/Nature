# 开发过程遇到的问题及解决方法

## nature-manager

### element-ui 与 vue 兼容性问题

必须使用 element-plus 版本

### npm run serve 报错

错误内容为：

> ERROR  ValidationError: Progress Plugin Invalid Options

使用初始创建的缺省依赖版本比较低，需要升级下面两个组件

- copy-webpack-plugin 升级到6以上

- webpack 升级到5以上

- webpack-cli 升级到 4 以上

- webpack-dev-server 升级到 4 以上

### Parsing error: No Babel config file detected for

解决方法：https://blog.csdn.net/weixin_43214005/article/details/123044266

在 package.json中增加配置：

> "requireConfigFile": false

### vue 支持typescript

需要引入下面的包

> "@vue/cli-plugin-typescript": "^5.0.4"

### vite 对 webAssembly 的支持

wasm-pack build 时需要加参数 --target web 

vite 报下面错误的解决方法：

> is outside of Vite serving allow list.

Vite config：

```
vite: {
  server: {
    fs: {
      // Allow serving files from one level up to the project root
      allow: ['..'],
    },
  },
},
```


