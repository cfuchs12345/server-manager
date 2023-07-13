import { createAction, props } from '@ngrx/store';
import { Update } from '@ngrx/entity';
import { Status } from '../../services/servers/types';

export const STATUS_UPDATE_ACTION = '[Status] Update';
export const STATUS_INSERT_ACTION = '[Status] Insert';
export const STATUS_INSERT_MANY_ACTION = '[Status] Insert Many';
export const STATUS_DELETE_ACTION = '[Status] Delete';
export const STATUS_INSERT_OR_UPDATE_ACTION = '[Status] Insert or Update';


export const STATUS_EFFECT_LOAD_ALL = '[Status] Effect: Load All';
export const STATUS_EFFECT_LOAD_ALL_SUCCESS = '[Status] Effect: Load All Success';
export const STATUS_EFFECT_LOAD_ALL_FAILURE = '[Status] Effect: Load All Failure';



export const updateOne = createAction(STATUS_UPDATE_ACTION, props<{ status: Update<Status> }>());
export const addOne = createAction(STATUS_INSERT_ACTION, props<{ status: Status }>());
export const addMany = createAction(STATUS_INSERT_MANY_ACTION, props<{ status: Status[] }>());
export const removeOne = createAction(STATUS_DELETE_ACTION, props<{ ipaddress: string }>());


export const upsertOne = createAction(STATUS_INSERT_OR_UPDATE_ACTION, props<{ status: Status }>());


// for effects
export const loadAll = createAction(STATUS_EFFECT_LOAD_ALL);
export const loadAllSuccess = createAction(STATUS_EFFECT_LOAD_ALL_SUCCESS, props<{status: Status[]}>());
export const loadAllFailure = createAction(STATUS_EFFECT_LOAD_ALL_FAILURE, props<{error: Error}>());
