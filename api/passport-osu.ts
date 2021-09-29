import OAuth2Strategy from "passport-oauth2";
import axios from "axios";
import { AuthenticateOptions } from "passport";
import { RequestHandler } from "express";

import { User} from "~/api/models/user";

const options: OAuth2Strategy.StrategyOptions = {
  authorizationURL: "https://osu.ppy.sh/oauth/authorize",
  tokenURL: "https://osu.ppy.sh/oauth/token",

  // TODO: have some kind of config checker so we don't need !
  // should be fine for now since it'll basically blow up
  clientID: process.env["THUB_OSU_CLIENT_ID"]!,
  clientSecret: process.env["THUB_OSU_CLIENT_SECRET"]!,
  callbackURL: process.env["THUB_PUBLIC_URL"] + "/api/callback",
};

const verify = async (
  accessToken: string,
  refreshToken: string,
  profile: any,
  cb: OAuth2Strategy.VerifyCallback
) => {
  let res = await axios.get("https://osu.ppy.sh/api/v2/me", {
    headers: {
      authorization: `Bearer ${accessToken}`,
    },
  });
  console.log("HELO", profile, res.data);

  let user = await User.findOne();

  // user doesn't exist in the database yet
  if (!user) {
  }

  cb(null);
};

export class OsuStrategy extends OAuth2Strategy {
  public name: string = "osu";

  constructor() {
    super(options, verify);
  }
}

export let callbackOptions: AuthenticateOptions = {
  successRedirect: "/",
};

export let callbackFunction: RequestHandler = (req, res) => {
  res.redirect("/");
};
