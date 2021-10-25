CREATE TABLE "users" (
  "osu_id" INTEGER PRIMARY KEY,
  "username" TEXT NOT NULL,
  "country_code" TEXT NOT NULL,
  "rank" INTEGER,
  "pp" FLOAT,
  "access_token" TEXT NOT NULL,
  "refresh_token" TEXT NOT NULL
);

CREATE TABLE "tournaments" (
  "id" INTEGER PRIMARY KEY AUTOINCREMENT,
  "name" VARCHAR(255) NOT NULL,
  "admin_id" INTEGER NOT NULL,
  "published" BOOLEAN NOT NULL DEFAULT FALSE,
  "format" VARCHAR(255),
  "mode_bitflags" INTEGER,
  "registrations_open" BOOLEAN,
  "rank_low_cap" INTEGER,
  "rank_high_cap" INTEGER,
  "country_restriction" JSON,
  "about" TEXT,
  "homepage" TEXT,
  "banner_image" VARCHAR(255),
  "createdAt" DATETIME NOT NULL,
  "updatedAt" DATETIME NOT NULL,

  FOREIGN KEY ("admin_id") REFERENCES "users" ("osu_id") ON DELETE NO ACTION ON UPDATE CASCADE
);
