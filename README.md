# CSS Random Color Generator
This is my first attempt on web assembly using Rust. It's a random color generator using css syntax. There are 3 of them which are HSL, RGB, and HEX.

## Setup using docker
To run this app, you must pull the docker image first. To do that, simply run this commands.

```
$ docker pull secondstate/ssvm-nodejs-starter:v1
```

## Run
Run this app by running these commands.

```
$ docker run -p 3000:3000 --rm -it -v $(pwd):/app secondstate/ssvm-nodejs-starter:v1
(docker) # cd /app
(docker) # ssvmup build
(docker) # node node/app.js
```

## Usage
There are 3 types of colours which are accessible through different query params. Here's a few example.

```
GET http://localhost:3000/?max=2&mode=hsl
```
Should return 2 hsl formatted colours like `hsl(12, 50, 30)` and `hsl(40, 22, 49)`

```
GET http://localhost:3000/?max=1&mode=rgb
```
Should return 1 rgb formatted colours like `rgb(230, 122, 43)`

```
GET http://localhost:3000/?max=4&mode=hex
```
Should return 1 rgb formatted colours like `#b0af3d`, `#c2ff3a`, `#02f01d`
