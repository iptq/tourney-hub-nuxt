export default {
  // Global page headers: https://go.nuxtjs.dev/config-head
  head: {
    title: "tourney-hub",
    htmlAttrs: {
      lang: "en",
    },
    meta: [
      { charset: "utf-8" },
      { name: "viewport", content: "width=device-width, initial-scale=1" },
      { hid: "description", name: "description", content: "" },
      { name: "format-detection", content: "telephone=no" },
    ],
    link: [{ rel: "icon", type: "image/x-icon", href: "/favicon.ico" }],
  },

  // Global CSS: https://go.nuxtjs.dev/config-css
  css: ["~/assets/styles"],

  // Plugins to run before rendering page: https://go.nuxtjs.dev/config-plugins
  plugins: ["~/plugins/hello"],

  // Auto import components: https://go.nuxtjs.dev/config-components
  components: true,

  // Modules for dev and build (recommended): https://go.nuxtjs.dev/config-modules
  buildModules: [
    // https://go.nuxtjs.dev/typescript
    "@nuxt/typescript-build",
  ],

  // Modules: https://go.nuxtjs.dev/config-modules
  modules: [
    // https://go.nuxtjs.dev/axios
    "@nuxtjs/axios",
    "@nuxtjs/dotenv",
    "@nuxtjs/auth-next",
  ],

  // Auth
  auth: {
    strategies: {
      cookie: {
        cookie: {
          name: "XSRF-TOKEN",
        },
        endpoints: {
          csrf: {
            url: "/api/auth/csrf",
          },
        },
      },
    },
  },

  // Axios module configuration: https://go.nuxtjs.dev/config-axios
  axios: {
    proxy: true,
  },

  // Build Configuration: https://go.nuxtjs.dev/config-build
  build: {
    analyze: true,
  },

  // Proxy for the API
  proxy: {
    // TODO: actually fetch this from the config
    "/api/": "http://localhost:3002",
  },

  server: {
    port: 3001,
  },
};
