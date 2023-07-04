import { createAction, props } from '@ngrx/store';
import { UserToken } from '../../services/users/types';
import { Update } from '@ngrx/entity';

export const USERTOKEN_UPDATE_ACTION = '[UserToken] Update';
export const USERTOKEN_INSERT_ACTION = '[UserToken] Insert';
export const USERTOKEN_DELETE_ACTION = '[UserToken] Delete';
export const USERTOKEN_INSERT_OR_UPDATE_ACTION = '[UserToken] Insert or Update';


export const updateOne = createAction(USERTOKEN_UPDATE_ACTION, props<{ usertoken: Update<UserToken> }>());
export const addOne = createAction(USERTOKEN_INSERT_ACTION, props<{ usertoken: UserToken }>());
export const removeOne = createAction(USERTOKEN_DELETE_ACTION, props<{ user_id: string }>());


export const upsertOne = createAction(USERTOKEN_INSERT_OR_UPDATE_ACTION, props<{ usertoken: UserToken }>());

