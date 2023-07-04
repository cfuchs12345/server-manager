import { createAction, props } from "@ngrx/store";
import { UserToken } from "../services/users/types";


export const init = createAction("[Global] init",  props<{ userToken: UserToken }>());

export const logout = createAction("[Global] logout",  props<{ userToken: UserToken }>());
