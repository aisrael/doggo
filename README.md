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
$ doggo -v metric post test.metric 1337 --type gauge --host localhost
POST https://api.datadoghq.com/api/v1/series
{"series":[{"host":"localhost","metric":"test.metric","points":[[1597398038,"1337"]],"tags":[],"type":"gauge"}]}
{"status": "ok"}
```

### Submit a metric (with tags)

```
$ doggo -v metric post test.metric 1337 --type gauge --host localhost -t source:doggo -t foo
POST https://api.datadoghq.com/api/v1/series
{"series":[{"host":"localhost","metric":"test.metric","points":[[1597398073,"1337"]],"tags":["source:doggo","foo"],"type":"gauge"}]}
{"status": "ok"}
```
