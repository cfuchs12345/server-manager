import { createAction, props } from "@ngrx/store";
import { UserToken } from "../services/users/types";


export const init = createAction("[Global] init",  props<{ userToken: UserToken }>());

export const logout = createAction("[Global] logout",  props<{ userToken: UserToken, logout: boolean }>());

export const logoutDone = createAction("[Global] logout done");


export const loadAll = createAction("[Global] load all");
export const loadDone = createAction("[Global] load all done");




