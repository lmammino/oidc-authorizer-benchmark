# oidc-authorizer-benchmark

A repo to benchmark https://github.com/lmammino/oidc-authorizer

Heavily WIP 😅


## Log Insights report

### Query

```plain
filter @type = "REPORT"
| stats
  
  # Cold Start
  sum(strcontains(@message, "Init Duration")) as numColdStarts,
  count(*) as totalRequests,
  sum(strcontains(@message, "Init Duration"))/count(*) * 100 as coldStartPct,
  avg(@initDuration) as AvgInitDurationMs,
  percentile(@initDuration, 99) as p99InitDurationMs,
  percentile(@initDuration, 95) as p95InitDurationMs,
  percentile(@initDuration, 90) as p90InitDurationMs,
  min(@initDuration) as minInitDurationMs,
  max(@initDuration) as maxInitDurationMs,
  sum(@initDuration) as totalInitDurationMs,
  
  # Duration
  avg(@duration) as AvgDurationMs,
  percentile(@duration, 99) as p99DurationMs,
  percentile(@duration, 95) as p95DurationMs,
  percentile(@duration, 90) as p90DurationMs,
  min(@duration) as minDurationMs,
  max(@duration) as maxDurationMs,
  sum(@duration) as totalDurationMs,

  # Billed Duration
  avg(@billedDuration) as AvgBilledDurationMs,
  percentile(@billedDuration, 99) as p99BilledDurationMs,
  percentile(@billedDuration, 95) as p95BilledDurationMs,
  percentile(@billedDuration, 90) as p90BilledDurationMs,
  min(@billedDuration) as minBilledDurationMs,
  max(@billedDuration) as maxBilledDurationMs,
  sum(@billedDuration) as totalBilledDurationMs,

  # Memory Used
  avg(@maxMemoryUsed / 1000 / 1000) as avgMemoryUsedMB,
  percentile(@maxMemoryUsed / 1000 / 1000, 99) as p99MemoryUsedMB,
  percentile(@maxMemoryUsed / 1000 / 1000, 95) as p95MemoryUsedMB,
  percentile(@maxMemoryUsed / 1000 / 1000, 90) as p90MemoryUsedMB,
  min(@maxMemoryUsed / 1000 / 1000) as minMemoryUsedMB,
  max(@maxMemoryUsed / 1000 / 1000) as maxMemoryUsedMB,

  # Cost (price per 128MB in eu-west-1: ~$0.0000000016671875)
  sum(@billedDuration) as TotalExecutionMs,
  avg(@memorySize / 1000 / 1000) as MemorySizeMB,
  (avg(@memorySize / 1000 / 1000) / 128) * 0.0000000016671875 as CostUsdMs,
  CostUsdMs * TotalExecutionMs as TotalCostUsd
by @log
```

<details>

<summary>🔎 Query results</summary>

**CloudWatch Logs Insights**    
region: eu-west-1    
log-group-names: /aws/lambda/oidc-authorizer-benchmark-oidcautho-OidcAuthorizer-W6chhahRMPKa, /aws/lambda/oidc-authorizer-benchmark-PythonOidcAuthorizer-28t3w9aiqqW9    
start-time: -3600s    
end-time: 0s    
query-string:
  ```
  filter @type = "REPORT"
| stats
  
  # Cold Start
  sum(strcontains(@message, "Init Duration")) as numColdStarts,
  count(*) as totalRequests,
  sum(strcontains(@message, "Init Duration"))/count(*) * 100 as coldStartPct,
  avg(@initDuration) as AvgInitDurationMs,
  percentile(@initDuration, 99) as p99InitDurationMs,
  percentile(@initDuration, 95) as p95InitDurationMs,
  percentile(@initDuration, 90) as p90InitDurationMs,
  min(@initDuration) as minInitDurationMs,
  max(@initDuration) as maxInitDurationMs,
  sum(@initDuration) as totalInitDurationMs,
  
  # Duration
  avg(@duration) as AvgDurationMs,
  percentile(@duration, 99) as p99DurationMs,
  percentile(@duration, 95) as p95DurationMs,
  percentile(@duration, 90) as p90DurationMs,
  min(@duration) as minDurationMs,
  max(@duration) as maxDurationMs,
  sum(@duration) as totalDurationMs,

  # Billed Duration
  avg(@billedDuration) as AvgBilledDurationMs,
  percentile(@billedDuration, 99) as p99BilledDurationMs,
  percentile(@billedDuration, 95) as p95BilledDurationMs,
  percentile(@billedDuration, 90) as p90BilledDurationMs,
  min(@billedDuration) as minBilledDurationMs,
  max(@billedDuration) as maxBilledDurationMs,
  sum(@billedDuration) as totalBilledDurationMs,

  # Memory Used
  avg(@maxMemoryUsed / 1000 / 1000) as avgMemoryUsedMB,
  percentile(@maxMemoryUsed / 1000 / 1000, 99) as p99MemoryUsedMB,
  percentile(@maxMemoryUsed / 1000 / 1000, 95) as p95MemoryUsedMB,
  percentile(@maxMemoryUsed / 1000 / 1000, 90) as p90MemoryUsedMB,
  min(@maxMemoryUsed / 1000 / 1000) as minMemoryUsedMB,
  max(@maxMemoryUsed / 1000 / 1000) as maxMemoryUsedMB,

  # Cost (price per 128MB in eu-west-1: ~$0.0000000016671875)
  sum(@billedDuration) as TotalExecutionMs,
  avg(@memorySize / 1000 / 1000) as MemorySizeMB,
  (avg(@memorySize / 1000 / 1000) / 128) * 0.0000000016671875 as CostUsdMs,
  CostUsdMs * TotalExecutionMs as TotalCostUsd
by @log
  ```
---
| @log | numColdStarts | totalRequests | coldStartPct | AvgInitDurationMs | p99InitDurationMs | p95InitDurationMs | p90InitDurationMs | minInitDurationMs | maxInitDurationMs | totalInitDurationMs | AvgDurationMs | p99DurationMs | p95DurationMs | p90DurationMs | minDurationMs | maxDurationMs | totalDurationMs | AvgBilledDurationMs | p99BilledDurationMs | p95BilledDurationMs | p90BilledDurationMs | minBilledDurationMs | maxBilledDurationMs | totalBilledDurationMs | avgMemoryUsedMB | p99MemoryUsedMB | p95MemoryUsedMB | p90MemoryUsedMB | minMemoryUsedMB | maxMemoryUsedMB | TotalExecutionMs | MemorySizeMB | CostUsdMs | TotalCostUsd |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 208950529517:/aws/lambda/oidc-authorizer-benchmark-PythonOidcAuthorizer-28t3w9aiqqW9 | 238 | 10000 | 2.38 | 642.9342 | 685.9023 | 677.0477 | 670.9846 | 583.75 | 718.65 | 153018.34 | 14.1354 | 333.501 | 15.5816 | 14.1661 | 2.02 | 437.05 | 141353.55 | 14.6481 | 338 | 16 | 15 | 3 | 438 | 146481 | 74.9728 | 75 | 75 | 75 | 74 | 78 | 146481 | 256 | 0.000000003334 | 0.0004884 |
| 208950529517:/aws/lambda/oidc-authorizer-benchmark-oidcautho-OidcAuthorizer-W6chhahRMPKa | 68 | 9799 | 0.6939 | 40.2175 | 70.36 | 42.03 | 41.45 | 37.19 | 70.36 | 2734.79 | 2.7125 | 16.0842 | 9.5259 | 5.9169 | 0.98 | 76.7 | 26579.6 | 3.691 | 17 | 10 | 6 | 1 | 127 | 36168 | 21.9338 | 22 | 22 | 22 | 19 | 23 | 36168 | 256 | 0.000000003334 | 0.0001206 |
---

</details>
