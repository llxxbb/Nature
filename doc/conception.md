# Conception

Nature accept `Thing`s and give it to `Converter` you designed, then `Converter` return new `Thing`s again to `Nature`. 
that will be continue until there is no `Converter` to handle the new generated `Thing`s.

So `Nature` is a Caller and `Converter`s are Functions and `Thing`s are input and output parameters.

The core work of `Nature` is to call `Converter` in the order specified and make sure the accepted and output `Thing`s in consistent way. 
This will great simplify you work and make it fast, correct and easy to maintain.

## How

### Just select no control
 
Control is complex, so `Nature` do it for you. you just provide base function(`Converter`) to `Nature`, 
you can __SELECT__ what `Thing` your `Converter` to process and generate what `Thing`s you want, 
and `Nature` will organize your `Converter`s in turn to work out your requirement.

### Idempotent

The same input `Thing` will get same `Thing`s output.

### Trace



## Thing

They are your business that will be inputted to and output by `Nature`.

They can be anything your want to be processed, like 'Order', 'Sheet', 'Check', 'Apply' and other things.

## Converter

### static converter

Converter Configuration must be added to `one_step_flow` table, so that it can be loaded and the `Nature` could send tasks to it.

###  dynamic-converter

You can dispatch you task at runtime for any target undefined.
    
__Notice__ dynamic-thing can only use dynamic-converter and only can generate dynamic-thing.