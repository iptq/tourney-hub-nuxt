import OAuth2Strategy from "passport-oauth2";

export default new OAuth2Strategy(
  {
    authorizationURL: "https://osu.ppy.sh/oauth/authorize",
    tokenURL: "https://osu.ppy.sh/oauth/token",
    clientID: "TODO",
    clientSecret: "TODO",
    callbackURL: "TODO",
  },
  (accessToken, refreshToken, profile, cb) => {}
);
