const { generate_colors } = require("../pkg/ssvm_nodejs_starter_lib.js")

const http = require("http")
const url = require("url")
const hostname = "0.0.0.0"
const port = 3000

const server = http.createServer((req, res) => {
  const queryObject = url.parse(req.url, true).query
  const { mode, max } = queryObject
  res.end(generate_colors(parseInt(max), mode) + "\n")
})

server.listen(port, hostname, () => {
  console.log(`Server running at http://${hostname}:${port}/`)
})
