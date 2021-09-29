import Koa from "koa";
import BodyParser from "koa-bodyparser";

const app = new Koa();

app.use(BodyParser());

module.exports = { handler: app };
