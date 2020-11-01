# Compare with other framework

## map-reduce framework(MRF)

### use level

* MRF's data are defined at low level(by programer).
* Nature's data are defined at high level by businness manager.

### use case

* MRF used in OLAP
* Nature used in BP

## compare to MQ

### Nature:

Easy and union redo ability.except finst call from outer all following process will automatic retry when **Environment Error** occurs.
Even so the first call from outer which wuould be UI usually, so no need retry logic at all.



### business relation

Find relation Forward versus backward.

static relation versus dynamic relation.


