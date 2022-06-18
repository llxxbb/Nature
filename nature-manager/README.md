<div align="center">

# Nature Manager

## how to use

you can change mode by press: **Ctrl + [key]**, where the [key] is defined below:

| key | meaning       |
| --- | ------------- |
| d   | Domain Mode   |
| r   | Relation Mode |
| i   | Instance Mode |

</div>

## Technology stack

[rust wasm-pack](https://github.com/rustwasm/wasm-pack)

[Element-plus](https://element-plus.gitee.io/zh-CN/), Notice: not element-ui, element-ui suit for vue2

[Vue.js](https://cn.vuejs.org/), here we use vue 3

**icon**

use package @iconify-json/ep", example : 

```html
<i inline-flex i="dark:ep-moon ep-sunny" />
```

`dark:ep-moon` and  `ep-sunny` are the icons we used to change theme.

## Make wasm package

wasm-pack build --target web

## Directory

- /pkg : store built WebAssembly

- /www: store the pages for web

## start the app

cd www
npm run serve

## About WebAssembly

This template is designed for compiling Rust libraries into WebAssembly and
publishing the resulting package to NPM.

Be sure to check out [other `wasm-pack` tutorials online][tutorials] for other
templates and usages of `wasm-pack`.

[tutorials]: https://rustwasm.github.io/docs/wasm-pack/tutorials/index.html
[template-docs]: https://rustwasm.github.io/docs/wasm-pack/tutorials/npm-browser-packages/index.html

## Usage

### Use `cargo generate` to Clone this Template

[Learn more about `cargo generate` here.](https://github.com/ashleygwilliams/cargo-generate)

```
cargo generate --git https://github.com/rustwasm/wasm-pack-template.git --name my-project
cd my-project
```

### Build with `wasm-pack build`

```
wasm-pack build
```

### Test in Headless Browsers with `wasm-pack test`

```
wasm-pack test --headless --firefox
```

### Publish to NPM with `wasm-pack publish`

```
wasm-pack publish
```

## Batteries Included

* [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
* [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)
  for logging panic messages to the developer console.
* [`wee_alloc`](https://github.com/rustwasm/wee_alloc), an allocator optimized
  for small code size.
