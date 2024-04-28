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

<details>

<summary>


#### Initial results

</summary>

**CloudWatch Logs Insights**    
region: eu-west-1    
log-group-names: /aws/lambda/oidc-authorizer-benchmark-oidcautho-OidcAuthorizer-WCH68cPWb0DB    
start-time: -1800s    
end-time: 0s    
query-string:
  ```
  filter @type = "REPORT"
| stats
  avg(@billedDuration) as Average,
  percentile(@billedDuration, 99) as NinetyNinth,
  percentile(@billedDuration, 95) as NinetyFifth,
  percentile(@billedDuration, 90) as Ninetieth
by bin(30m)
  ```
---
| bin(30m) | Average | NinetyNinth | NinetyFifth | Ninetieth |
| --- | --- | --- | --- | --- |
| 2024-04-28 16:00:00.000 | 4.7965 | 91 | 11 | 8 |
---

</details>



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

<details>

<summary>

#### Initial results

</summary>

**CloudWatch Logs Insights**    
region: eu-west-1    
log-group-names: /aws/lambda/oidc-authorizer-benchmark-oidcautho-OidcAuthorizer-WCH68cPWb0DB    
start-time: -1800s    
end-time: 0s    
query-string:
  ```
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
---
| bin(30m) | coldStartPct | AvgDuration | NinetyNinth | NinetyFifth | Ninetieth | numColdStart | totalRequests |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 2024-04-28 16:00:00.000 | 1.93 | 3.3531 | 51.2436 | 10.4778 | 6.9348 | 193 | 10000 |
---

</details>



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

<details>

<summary>

#### Initial results

</summary>

**CloudWatch Logs Insights**    
region: eu-west-1    
log-group-names: /aws/lambda/oidc-authorizer-benchmark-oidcautho-OidcAuthorizer-WCH68cPWb0DB    
start-time: -1800s    
end-time: 0s    
query-string:
  ```
  filter @type = "REPORT"
| stats
  sum(@billedDuration) as TotalExecutionMs,
  avg(@memorySize) as MemorySize,
  avg(@maxMemoryUsed) as AvgMaxMemoryUsed,
  (TotalExecutionMs * 0.0000000042) as TotalCostUSD
by bin(30m)
  ```
---
| bin(30m) | TotalExecutionMs | MemorySize | AvgMaxMemoryUsed | TotalCostUSD |
| --- | --- | --- | --- | --- |
| 2024-04-28 16:00:00.000 | 47965 | 256000000 | 21886900 | 0.0002015 |
---

</details>