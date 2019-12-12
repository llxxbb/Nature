# Relation



# converter

`converter` 用于实现 `Meta` 间 `Instance` 的转换，一般需要自己实现，Nature 也有内建及自动化的 `converter` 实现。

## 支持的通讯协议

说明 Nature 是以何种方式同 `converter`进行通信。目前支持下面的方式

- http | https
- local rust

## 注册一个 `converter`

`converter` 需要添加到 `relation`数据表后才能被 Nature  所调用。

## 如何实现一个 `converter`

`converter` 是面向业务的，没有技术上的难点，`converter`会接收一个``类型的输入 you will only concern about one input-parameter : `meta`'s `instance` and generate one or more output `instance`s

## static converter (Static Orchestration)

Converter Configuration must be added to `relation` table, so that it can be loaded before process `instance`s .In this way the  `relation` can be cached so it's efficient, 

## dynamic-converter (Dynamic Orchestration)

You can dispatch you task at runtime for any downstream `meta` undefined. In this way you need provide `converter` in every inputted `instance`, It would spend more time than `Static Orchestration`, but it's flexible.

__Notice__ dynamic-meta can only use dynamic-converter and only can generate dynamic-meta (see [Meta](meta.md)).



