# oidc-authorizer-benchmark

A repo to benchmark https://github.com/lmammino/oidc-authorizer

Heavily WIP 😅


## Log Insights useful queries

TODO: add them as pre-defined queries in the log insights console through SAM:

### Lambda execution billed duration

```plain
filter @type = "REPORT"
| stats
  avg(@billedDuration) as Average,
  percentile(@billedDuration, 99) as NinetyNinth,
  percentile(@billedDuration, 95) as NinetyFifth,
  percentile(@billedDuration, 90) as Ninetieth
by bin(30m)
```

### Cold Start Percentage and Average Duration

```plain
filter @type = "REPORT"
| stats sum(strcontains(@message, "Init Duration"))/count(*) * 100 as coldStartPct, 
  avg(@duration) as AvgDuration,
  percentile(@duration, 99) as NinetyNinth,
  percentile(@duration, 95) as NinetyFifth,
  percentile(@duration, 90) as Ninetieth,
  sum(strcontains(@message, "Init Duration")) as numColdStart, 
  count(*) as totalRequests
by bin(30m)
```

### Execution cost

`eu-west-1` pricing:
  - **128MB**: `$0.0000000017`
  - **256MB**: `$0.0000000042` (estimated)
  - **512MB**: `$0.0000000067`

```plain
filter @type = "REPORT"
| stats
  sum(@billedDuration) as TotalExecutionMs,
  avg(@memorySize) as MemorySize,
  avg(@maxMemoryUsed) as AvgMaxMemoryUsed,
  (TotalExecutionMs * 0.0000000042) as TotalCostUSD
by bin(30m)
```