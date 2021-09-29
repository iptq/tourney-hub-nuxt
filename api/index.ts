import express from "express";
import passport from "passport";
import session from "express-session";

// import koa from "koa";
// import koaBodyParser from "koa-bodyparser";
// import koaSession from "koa-session";
// import koaPassport from "koa-passport";
// import koaRouter from "@koa/router";

import osuStrategy from "./passport-osu";

const app = express();

app.use(
  session({
    secret: "TODO",
    saveUninitialized: false,
  })
);

app.use(passport.initialize());
app.use(passport.session());
passport.use("osu", osuStrategy);

app.get("/login", passport.authenticate("osu", {}));
app.get("/asdf", (req, res) => {
  res.send("hellosu");
});

module.exports = app;
