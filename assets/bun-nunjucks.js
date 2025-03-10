import nunjucks from "nunjucks"

export default {
  name: "nunjucks",
  setup(build) {
    nunjucks.configure('templates');

    build.onLoad({ filter: /.*.html/, namespace: "file" }, async ({ path }) => {
      const contents = nunjucks.render(path, { foo: "FOO" });
      return {
        contents: contents,
        loader: "html",
      };
    });

  },
};

