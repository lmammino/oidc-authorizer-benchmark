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



### Cold Start Percentage and Average init Duration

```plain
filter @type = "REPORT"
| stats sum(strcontains(@message, "Init Duration"))/count(*) * 100 as coldStartPct, 
  avg(@initDuration) as AvgDuration,
  percentile(@initDuration, 99) as NinetyNinth,
  percentile(@initDuration, 95) as NinetyFifth,
  percentile(@initDuration, 90) as Ninetieth,
  sum(strcontains(@initDuration, "Init Duration")) as numColdStart, 
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
start-time: -3600s    
end-time: 0s    
query-string:
  ```
  filter @type = "REPORT"
| stats sum(strcontains(@message, "Init Duration"))/count(*) * 100 as coldStartPct, 
  avg(@initDuration) as AvgDuration,
  percentile(@initDuration, 99) as NinetyNinth,
  percentile(@initDuration, 95) as NinetyFifth,
  percentile(@initDuration, 90) as Ninetieth,
  sum(strcontains(@initDuration, "Init Duration")) as numColdStart, 
  count(*) as totalRequests
by bin(30m)
  ```
---
| bin(30m) | coldStartPct | AvgDuration | NinetyNinth | NinetyFifth | Ninetieth | numColdStart | totalRequests |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 2024-04-28 16:00:00.000 | 1.93 | 39.2652 | 45.0646 | 41.5599 | 40.5749 | 0 | 10000 |
| 2024-04-28 15:30:00.000 | 0 |  | 0 | 0 | 0 |  | 1 |
---

</details>



### Execution cost

[`eu-west-1` pricing](https://aws.amazon.com/lambda/pricing/):

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