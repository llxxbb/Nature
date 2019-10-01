# converter

`converter` is a logical outside of Nature. 

## Supported protocols

it can be implemented in many forms.  up to now Nature support the following forms

- http | https
- local rust

## How to implement a converter

`converter` is easy to implement, you will only concern about one input-parameter : `meta`'s `instance` and generate one or more output `instance`s

## static converter (Static Orchestration)

Converter Configuration must be added to `relation` table, so that it can be loaded before process `instance`s .In this way the  `relation` can be cached so it's efficient, 

## dynamic-converter (Dynamic Orchestration)

You can dispatch you task at runtime for any downstream `meta` undefined. In this way you need provide `converter` in every inputted `instance`, It would spend more time than `Static Orchestration`, but it's flexible.

__Notice__ dynamic-meta can only use dynamic-converter and only can generate dynamic-meta (see [Meta](concept-meta.md)).



