// @ts-check

import { createServer } from "node:http";

const server = createServer((req, res) => {
  const fwd = req.headers["X-Tuc-Fwd-For"] || "<no `X-Tuc-Fwd-For`>";
  res.end(`hello, world! (fwd by ${fwd})`);
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
