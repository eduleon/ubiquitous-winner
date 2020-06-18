__Lightning fast app__ base on _Actix_ (TOP#2 Web Framework by Performance -> https://www.techempower.com/benchmarks/#section=data-r19&hw=ph&test=composite)

# ALT1: Rust Actix-Web app

## Install Rust (depends on OS)
https://www.rust-lang.org/tools/install
## Running unit tests
```bash
cargo test
```

## Run app
```bash
cargo run
# Started http server: localhost:5000
```

# ALT2: Dockerized Actix-Web app

## Build image _(be pacient)_

```shell
docker build -t actix-web-sample:1.0 .
```

## Running unit tests

```shell
docker build -t actix-web-sample:test --target base .
docker run --rm actix-web-sample:test
```

## Run built image

```shell
docker run -p 5000:5000 actix-web-sample:1.0
# and the server should start instantly
curl http://localhost:5000/
```

## Usage

- [http://localhost:5000/](http://localhost:5000/)
- [http://localhost:5000/sma?indicator=cobre&period=50](http://localhost:5000/sma?indicator=cobre&period=50)
- [http://localhost:5000/sma?indicator=plata&period=100](http://localhost:5000/sma?indicator=plata&period=100)
- [http://localhost:5000/sma?indicator=oro&period=200](http://localhost:5000/sma?indicator=oro&period=200)