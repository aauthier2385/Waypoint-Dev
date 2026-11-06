// A small web server with no dependencies. It sends files from the "public" folder.
const http = require("node:http");
const fs = require("node:fs");
const path = require("node:path");

const PORT = process.env.PORT || 3000;
const PUBLIC = path.join(__dirname, "public");
const TYPES = {
  ".html": "text/html; charset=utf-8",
  ".css": "text/css",
  ".js": "text/javascript",
  ".json": "application/json",
  ".webmanifest": "application/manifest+json",
  ".svg": "image/svg+xml",
};

const server = http.createServer((request, response) => {
  const urlPath = new URL(request.url, "http://localhost").pathname;
  const relative = urlPath === "/" ? "index.html" : urlPath.slice(1);
  const file = path.resolve(PUBLIC, relative);

  // Only ever send files from inside the public folder.
  if (file !== PUBLIC && !file.startsWith(PUBLIC + path.sep)) {
    response.writeHead(403).end("Not allowed");
    return;
  }
  fs.readFile(file, (error, contents) => {
    if (error) {
      response.writeHead(404).end("Not found");
      return;
    }
    response.writeHead(200, { "Content-Type": TYPES[path.extname(file)] || "application/octet-stream" });
    response.end(contents);
  });
});

server.listen(PORT, "127.0.0.1", () => {
  console.log(`Your app is running. Open http://localhost:${PORT} in your browser.`);
  console.log("Press Ctrl+C here to stop it.");
});
