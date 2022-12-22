# nature-manager-ui

You can use this project to manage Meta and Relation of [Nature](https://github.com/llxxbb/Nature)

Please user version 0.22.4 or above of Nature.

## Relation Mode

In this mode, you can organize the business how to work.

![main](doc/relation.png?raw=true)

- red label: the `Meta` has states
- black label: normal `Meta` which has no state

## Domain Mode

In this mode, you can organize the business domain.

![main](doc/domain.png?raw=true)

## Instance Mode

In this mode, you can see the data flow

![main](doc/instance.png?raw=true)

## Functions

- change mode by right-click at the blank of the layer.
- context-menu for node.
- can expand-collapse the nodes.
- show `State-Meta` text in red title
- show red circle for current selected node.
- for same `Meta`, use virtual circle to show repeated, and use blue color to identify them when move on them.
- use virtual circle to show `path-domain`.
- can modify settings in `config.ts`

## How to Use

```
npm install
```

### Run Backend

```shell
node backend/index.js
```

this will start web service at port : 3000 by default

### Run Client

```
npm run serve
```

this will start client, please visit: http://localhost:8280

### Compiles and minifies for production

```
npm run build
```

### Run your unit tests

```
npm run test:unit
```

### Customize configuration

See [Configuration Reference](https://cli.vuejs.org/config/).

## Change Logs

**0.9.0**

- add Tips

**0.8.0** 2021-03-21

- make color configurable
- show disabled `Meta` in gray bgColor
- show undefined `Meta` in pink bgColor
- show disabled `Relation` in grey
- show normal `Relation` in SaddleBrown

**0.7.0**  2021-02-18

- tooltip for `Meta`
- tooltip for `Instance`
- tooltip for `Relation`

**0.6.1** 2021-02-15

- list state-instance
- view instance detail
- show current instance detail
- bug fix: state not shown in red

**0.5.0** 2021-02-13

- recent instance
- config.js: support INSTANCE_RELATED_AUTO

**0.4.0** 2021-02-10

- support Instance-mode

**0.3.0** 2021-02-08

- Don't show instance query when node is not a real meta.
- bug fix: meta-context-menu and layout-context-menu conflict
- show State-Meta text in red
- can modify setting in `config.ts`
- don't show status-version input box when it's unnecessary
- optimize: put it back for a failed drag
- remove business from d3tree

**v0.2.0** 2021-01-24

can show business domain in Domain-Mode. you can do that by right click the blank of the layout, and click the corresponding menu item on the context-menu.

details:

- bug fix: layer context should not be shown when Meta Context is showing.
- can show Business Domain layout.
- use square to show Domain-Node.

**v0.1.1** 2021-01-23

- bug fix: change mock data to real data, now you see every thing which defined in tables,

**v0.1.0** 2021-01-19

- can show `Meta` and `Relation`, but can't add, edit etcetera.
