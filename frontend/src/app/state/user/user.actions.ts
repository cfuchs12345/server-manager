import { createAction, props } from '@ngrx/store';
import { Update } from '@ngrx/entity';
import { User } from '../../services/users/types';


export const USER_UPDATE_ACTION = '[User] Update';
export const USER_INSERT_ACTION = '[User] Insert';
export const USER_INSERT_MANY_ACTION = '[User] Insert Many';
export const USER_DELETE_ACTION = '[User] Delete';
export const USER_INSERT_OR_UPDATE_ACTION = '[User] Insert or Update';


export const USER_EFFECT_LOAD_ALL = '[User] Effect: Load All';
export const USER_EFFECT_LOAD_ALL_SUCCESS = '[User] Effect: Load All Success';
export const USER_EFFECT_LOAD_ALL_FAILURE = '[User] Effect: Load All Failure';


export const updateOne = createAction(USER_UPDATE_ACTION, props<{ user: Update<User> }>());
export const addOne = createAction(USER_INSERT_ACTION, props<{ user: User }>());
export const addMany = createAction(USER_INSERT_MANY_ACTION, props<{ users: User[] }>());
export const removeOne = createAction(USER_DELETE_ACTION, props<{ user_id: string }>());
export const upsertOne = createAction(USER_INSERT_OR_UPDATE_ACTION, props<{ user: User }>());


// for effects
export const loadAll = createAction(USER_EFFECT_LOAD_ALL);
export const loadAllSuccess = createAction(USER_EFFECT_LOAD_ALL_SUCCESS, props<{users: User[]}>());
export const loadAllFailure = createAction(USER_EFFECT_LOAD_ALL_FAILURE, props<{error: Error}>());
