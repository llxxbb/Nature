# 开发过程遇到的问题及解决方法

## nature-manager

### element-ui 与 vue 兼容性问题

必须使用 element-plus 版本

### npm run serve 报错

错误内容为：

> ERROR  ValidationError: Progress Plugin Invalid Options

使用初始创建的缺省依赖版本比较低，需要升级下面两个组件

-  copy-webpack-plugin 升级到6以上

- webpack 升级到5以上

- webpack-cli 升级到 4 以上

- webpack-dev-server 升级到 4 以上




