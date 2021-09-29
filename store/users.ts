import { ActionTree } from "vuex";

export interface UserInfo {}

export interface UserState {
  user: UserInfo | null;
}

export const actions: ActionTree<UserState, UserState> = {
  async login({ commit }, user) {
    commit("auth_request");
    let response = await this.$axios.$get("/api/login", {
      data: user,
      method: "POST",
    });
    commit("auth_success");
  },
};
