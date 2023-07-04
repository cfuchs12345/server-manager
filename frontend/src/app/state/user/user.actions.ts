import { createAction, props } from '@ngrx/store';
import { Update } from '@ngrx/entity';
import { User } from '../../services/users/types';

export const USEREXIST_INSERT_OR_UPDATE_ACTION = '[UserExist] Insert or Update';
export const USEREXIST_DELETE_ACTION = '[UserExist] Delete';

export const USER_UPDATE_ACTION = '[User] Update';
export const USER_INSERT_ACTION = '[User] Insert';
export const USER_INSERT_MANY_ACTION = '[User] Insert Many';
export const USER_DELETE_ACTION = '[User] Delete';
export const USER_INSERT_OR_UPDATE_ACTION = '[User] Insert or Update';


export const removeOneExist = createAction(USEREXIST_DELETE_ACTION);
export const upsertOneExist = createAction(USEREXIST_INSERT_OR_UPDATE_ACTION, props<{ exist: boolean }>());


export const updateOne = createAction(USER_UPDATE_ACTION, props<{ user: Update<User> }>());
export const addOne = createAction(USER_INSERT_ACTION, props<{ user: User }>());
export const addMany = createAction(USER_INSERT_MANY_ACTION, props<{ users: User[] }>());
export const removeOne = createAction(USER_DELETE_ACTION, props<{ user_id: string }>());
export const upsertOne = createAction(USER_INSERT_OR_UPDATE_ACTION, props<{ user: User }>());

