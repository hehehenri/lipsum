# lipsum

A three walking interpreter for [rinha de compilers](https://github.com/aripiprazole/rinha-de-compiler)

## Run it
```
$ docker build -t lipsum .

$ docker run -v \
    ./examples/hello-world.json:/var/rinha/source.rinha.json \
    --memory=2gb \
    --cpus=2 \
    lipsum
```
