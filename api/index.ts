import express from "express";
import passport from "passport";
import session from "express-session";
import { createConnection } from "typeorm";

import { OsuStrategy, callbackOptions, callbackFunction } from "./passport-osu";

const app = express();

// TODO: do an actual db
createConnection({
  type: "sqlite",
  database: "test.db",
}).then(async (_) => {
  console.log("Connected");

  app.use(
    session({
      secret: "TODO",
      saveUninitialized: false,
    })
  );

  app.use(passport.initialize());
  app.use(passport.session());

  passport.use(new OsuStrategy());

  app.get("/", (_, res) => {
    res.status(400);
    res.send("hi there");
  });

  app.get(
    "/callback",
    passport.authenticate("osu", callbackOptions),
    callbackFunction
  );
  app.get("/login", passport.authenticate("osu"));
});

module.exports = app;
