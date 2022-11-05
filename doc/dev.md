# 开发过程遇到的问题及解决方法

## nature-manager

### element-ui 与 vue 兼容性问题

必须使用 element-plus 版本

### element-plus 中的样式修改

[vue中令人头疼的element-ui组件默认css样式修改 - 掘金 (juejin.cn)](https://juejin.cn/post/7011016159545786376)

### 脚本中不能直接试用 assets 中的图片

需要先导入才可以使用。如

> import logo from "~/assets/logo.png";

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

### main.ts: Cannot find module './App.vue'

需要VS code 需要安装 TypeScript Vue Plugin (Volar)

### vetur could not find package.json

在项目根目录下放置 vetur.config.js 文件，内容如下：

module.exports = {

   projects: ["./www"]    // 指向子目录中的 package.json 所在目录

}

### 图像居中

[css如何让img垂直居中-css教程-PHP中文网](https://www.php.cn/css-tutorial-413180.html) 借鉴了其中的第二种方法

### eslint 报错

试用了下面的几种都有问题

    "@babel/eslint-parser": "^7.18.2",

    "@typescript-eslint/parser": "^5.27.1",

    "eslint-plugin-vue": "^9.1.0",

    "babel-eslint": "^10.1.0",
