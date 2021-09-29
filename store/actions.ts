import { ActionTree } from "vuex";
import { RootState } from "./.";

const actions: ActionTree<RootState, RootState> = {
  async login({ commit }, user) {
    commit("auth_request");
    let response = await this.$axios.$get("/api/login", { data: user, method: "POST" });
    commit("auth_success");
  },
};

export default actions;
