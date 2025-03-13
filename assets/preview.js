import page from "../templates/admin/events.html"

Bun.serve({
  routes: {
    "/": page,
  },
  development: {
    hmr: false,
  },
})

