// @ts-check

import { createServer } from "node:http";

const server = createServer((req, res) => {
  const fwd = req.headers["x-tuc-fwd-for"] || "no X-Tuc-Fwd-For";
  const inst = req.headers["x-tuc-inst"] || "no X-Tuc-Inst";
  res.end(`hello, world! (fwd by <${fwd}>) (inst <${inst}>)`);
});

const port = process.env.PORT;
server.listen(
  {
    port,
    host: "0.0.0.0",
  },
  () => {
    console.log(`listening at port ${port}`);
  }
);
