import fs from "fs";
import { defineConfig } from "vite";
export default defineConfig({
  plugins: [
    {
      name: "isolation",
      configureServer(server) {
        server.middlewares.use((_req, res, next) => {
          res.setHeader("Cross-Origin-Opener-Policy", "same-origin");
          res.setHeader("Cross-Origin-Embedder-Policy", "require-corp");
          next();
        });
      },
    },
  ],
//   server: {
//     https: {
//       key: fs.readFileSync("./cert/localhost-key.pem"),
//       cert: fs.readFileSync("./cert/localhost.pem"),
//     },
//   },
});