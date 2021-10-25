import { Module, VuexModule } from "vuex-module-decorators";

@Module({
  name: "users",
  stateFactory: true,
  namespaced: true,
})
export default class UserModule extends VuexModule {
  get isAuthenticated() {
    return true;
  }

  get loggedInUser() {
    return "Hellosu";
  }
}
