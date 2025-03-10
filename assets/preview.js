import page from "../templates/index.html"

Bun.serve({
  routes: {
    "/": page,
  },
  development: {
    hmr: false,
  },
})

