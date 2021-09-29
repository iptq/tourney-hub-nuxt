import Koa from "koa";
import BodyParser from "koa-bodyparser";
import portfinder from "portfinder";

const app = new Koa();

app.use(BodyParser());

module.exports = { handler: app };
