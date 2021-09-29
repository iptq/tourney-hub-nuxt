import type { ConnectionOptions } from "typeorm";

export default {
  type: "sqlite",
  database: "test.db",

  entities: [
    "api/models/**/*.ts",
  ],

  migrations: [
    "api/models/migrations/*.ts",
  ],

  cli: {
    migrationsDir: "api/models/migrations",
  },
} as ConnectionOptions;
