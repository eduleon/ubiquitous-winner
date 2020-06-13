# Docker sample

## Build image

```shell
docker build -t actix-web-sample:1.0 .
```

## Run built image

```shell
docker run -d -p 5000:5000 actix-web-sample
# and the server should start instantly
curl http://localhost:5000
```

## Running unit tests

```shell
docker build -t actix-web-sample:test --target base .
docker run --rm actix-web-sample:test
```
