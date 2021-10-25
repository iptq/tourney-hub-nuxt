CREATE TABLE "users" (
  "osu_id" INTEGER PRIMARY KEY,
  "username" TEXT NOT NULL,
  "country_code" TEXT NOT NULL,
  "rank" INTEGER,
  "pp" FLOAT,
  "access_token" TEXT NOT NULL,
  "refresh_token" TEXT NOT NULL
);
