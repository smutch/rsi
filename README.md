RSI
===

**R**usty **S**LURM **I**nfo

A (currently very) small collection of utils to ease working with SLURM and present information from `sacct` etc.


## Examples

Get a detailed output of the stats for a single step of a completed job:

```
rsi jobinfo 21941386
```

Estimate the start time of a job script the current state of the queue:

```
rsi starttime submit.sh
```
