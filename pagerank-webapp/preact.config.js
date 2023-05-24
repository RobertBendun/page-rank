module.exports = function (config) {
  config.devServer.proxy = [
    {
      // proxy requests matching a pattern:
      path: "/api/**",

      // where to proxy to:
      target: "http://localhost:8000",

      // optionally change Origin: and Host: headers to match target:
      changeOrigin: true,
      changeHost: true,

      // optionally mutate request before proxying:
      pathRewrite: function (path, request) {
        // you can modify the outbound proxy request here:
        delete request.headers.referer;

        // common: remove first path segment: (/api/**)
        return "/" + path.replace(/^\/[^\/]+\//, "");
      },

      // optionally mutate proxy response:
      onProxyRes: function (proxyRes, req, res) {
        // you can modify the response here:
        proxyRes.headers.connection = "keep-alive";
        proxyRes.headers["cache-control"] = "no-cache";
      },
    },
    {
      // proxy requests matching a pattern:
      path: "**/cppreference/**",

      // where to proxy to:
      target: "http://localhost:8081",

      // optionally change Origin: and Host: headers to match target:
      changeOrigin: true,
      changeHost: true,

      // optionally mutate request before proxying:
      pathRewrite: function (path, request) {
        // you can modify the outbound proxy request here:
        delete request.headers.referer;

        // common: remove first path segment: (/api/**)
        console.log("new target", "/" + path.replace(/.*cppreference/, ""));
        return "/" + path.replace(/.*cppreference/, "");
      },

      // optionally mutate proxy response:
      onProxyRes: function (proxyRes, req, res) {
        // you can modify the response here:
        proxyRes.headers.connection = "keep-alive";
        proxyRes.headers["cache-control"] = "no-cache";
      },
    },
  ];
};
