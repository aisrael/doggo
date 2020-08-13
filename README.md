doggo
===

A Datadog CLI client, written in Rust.

(Yes, I know, it would've punned better written in Go but...)


## Usage

### Check connectivity

Check if `doggo` can connect properly with a given `$DD_API_KEY`:


```
$ export DD_API_KEY=...
$ doggo authenticate
{"valid":true}
```

Alternatively, specify the Datadog API Key using `--api-key`:

```
$ doggo --api-key ... authenticate
{"valid":true}
```

### Submit a metric (with verbose output)

```
$ doggo -v metric post --host localhost --type gauge test.metric 1337
POST https://api.datadoghq.com/api/v1/series
{"series":[{"host":"localhost","metric":"test.metric","points":[[1597338519,"1337"]],"tags":"source:doggo","type":"gauge"}]}
{"status": "ok"}
```
