import { Module, VuexModule } from "vuex-module-decorators";

@Module({
  name: "users",
  stateFactory: true,
  namespaced: true,
})
export default class UserModule extends VuexModule {
  get isAuthenticated() {
    return false;
  }

  get loggedInUser() {
    return "Hellosu";
  }
}
