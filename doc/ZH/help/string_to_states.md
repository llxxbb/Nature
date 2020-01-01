# String convert to `State`s algorithm

## State-type

normal,mutex and parent.

## special characters

There are four special characters used to express the states:

| character | purpose                                |
| --------- | -------------------------------------- |
| ,         | separator for normal, mutex and parent |
| \|        | mutex normal and parent                |
| [         | parent begin                           |
| ]         | parent end                             |

They can embed to each other, for example "p[a,b|c[d,e|f]]"

## How to judge the end of a state-type

| type   | find begin              | find end                           |
| ------ | ----------------------- | ---------------------------------- |
| normal | occurs normal character | find "," or end of the stream      |
| mutex  | occurs "\|"             | find "," ,"]" or end of the stream |
| parent | occurs "["              | find "]"                           |

how to distinguish the end of the `normal` and `mutex`: when fine "|" mark it, when reached the end remove the mark

## Special pattern

| pattern | how                                      |
| ------- | ---------------------------------------- |
| ],      | add parent to array                      |
| ]\|     | mark mutex and add parent to mutex-state |
| ]]      | add parent to parent                     |

## process flow

process the character one by one of the string.



