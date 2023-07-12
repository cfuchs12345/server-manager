import { createAction, props } from '@ngrx/store';
import { Update } from '@ngrx/entity';
import { Server } from '../../services/servers/types';

export const SERVER_UPDATE_ACTION = '[Server] Update';
export const SERVER_INSERT_ACTION = '[Server] Insert';
export const SERVER_INSERT_MANY_ACTION = '[Server] Insert Many';
export const SERVER_DELETE_ACTION = '[Server] Delete';
export const SERVER_INSERT_OR_UPDATE_ACTION = '[Server] Insert or Update';

export const SERVER_EFFECT_LOAD_ALL = '[Server] Effect: Load All';
export const SERVER_EFFECT_LOAD_ALL_SUCCESS = '[Server] Effect: Load All Success';
export const SERVER_EFFECT_LOAD_ALL_FAILURE = '[Server] Effect: Load All Failure';

export const SERVER_EFFECT_LOAD_SPECIFIC = '[Server] Effect: Load Specific';
export const SERVER_EFFECT_LOAD_SPECIFIC_SUCCESS = '[Server] Effect: Load Specific Success';
export const SERVER_EFFECT_LOAD_SPECIFIC_FAILURE = '[Server] Effect: Load Specific Failure';

// for store update / reducers
export const updateOne = createAction(SERVER_UPDATE_ACTION, props<{ server: Update<Server> }>());
export const addOne = createAction(SERVER_INSERT_ACTION, props<{ server: Server }>());
export const addMany = createAction(SERVER_INSERT_MANY_ACTION, props<{ servers: Server[] }>());
export const removeOne = createAction(SERVER_DELETE_ACTION, props<{ ipaddress: string }>());
export const upsertOne = createAction(SERVER_INSERT_OR_UPDATE_ACTION, props<{ server: Server }>());


// for effects
export const loadAll = createAction(SERVER_EFFECT_LOAD_ALL);
export const loadAllSuccess = createAction(SERVER_EFFECT_LOAD_ALL_SUCCESS, props<{servers: Server[]}>());
export const loadAllFailure = createAction(SERVER_EFFECT_LOAD_ALL_FAILURE, props<{error: any}>());

export const loadSpecific = createAction(SERVER_EFFECT_LOAD_SPECIFIC, props<{ipaddress: string, fullData: boolean }>());
export const loadSpecificSuccess = createAction(SERVER_EFFECT_LOAD_SPECIFIC_SUCCESS, props<{server: Server}>());
export const loadSpecificFailure = createAction(SERVER_EFFECT_LOAD_SPECIFIC_FAILURE, props<{error: any}>());

