import { createAction, props } from '@ngrx/store';
import { Update } from '@ngrx/entity';
import { Server, ServerFeature } from 'src/app/services/servers/types';

export const SERVER_UPDATE_ACTION = '[Server] Update';
export const SERVER_INSERT_ACTION = '[Server] Insert';
export const SERVER_INSERT_MANY_ACTION = '[Server] Insert Many';
export const SERVER_DELETE_ACTION = '[Server] Delete';
export const SERVER_INSERT_OR_UPDATE_ACTION = '[Server] Insert or Update';

export const SERVER_EFFECT_LOAD_ALL = '[Server] Effect: Load All';
export const SERVER_EFFECT_LOAD_ALL_SUCCESS =
  '[Server] Effect: Load All Success';
export const SERVER_EFFECT_LOAD_ALL_FAILURE =
  '[Server] Effect: Load All Failure';

export const SERVER_EFFECT_LOAD_SPECIFIC = '[Server] Effect: Load Specific';
export const SERVER_EFFECT_LOAD_SPECIFIC_SUCCESS =
  '[Server] Effect: Load Specific Success';
export const SERVER_EFFECT_LOAD_SPECIFIC_FAILURE =
  '[Server] Effect: Load Specific Failure';

export const SERVER_EFFECT_SAVE_SERVER =
  '[Server] Effect: Save server';

export const SERVER_EFFECT_SAVE_SERVERS =
  '[Server] Effect: Save servers';

export const SERVER_EFFECT_REMOVE_SERVERS =
  '[Server] Effect: Remove servers';

export const SERVER_EFFECT_ADD_SERVER_FEATURE =
  '[Server] Effect: Add server feature';
export const SERVER_EFFECT_ADD_SERVER_FEATURE_SUCCESS =
  '[Server] Effect: Add server feature success';

export const SERVER_EFFECT_ADD_SERVER_FEATURES =
  '[Server] Effect: Add mutiple server features';
export const SERVER_EFFECT_ADD_SERVER_FEATURES_SUCCESS =
  '[Server] Effect: Add mutiple server features success';

export const SERVER_EFFECT_REMOVE_SERVER_FEATURE =
  '[Server] Effect: Remove feature';
export const SERVER_EFFECT_REMOVE_SERVER_FEATURE_SUCCESS =
  '[Server] Effect: Remove feature success';

// for store update / reducers
export const updateOne = createAction(
  SERVER_UPDATE_ACTION,
  props<{ server: Update<Server> }>()
);
export const addOne = createAction(
  SERVER_INSERT_ACTION,
  props<{ server: Server }>()
);
export const addMany = createAction(
  SERVER_INSERT_MANY_ACTION,
  props<{ servers: Server[] }>()
);
export const removeOne = createAction(
  SERVER_DELETE_ACTION,
  props<{ ipaddress: string }>()
);
export const upsertOne = createAction(
  SERVER_INSERT_OR_UPDATE_ACTION,
  props<{ server: Server }>()
);

// for effects
export const loadAll = createAction(SERVER_EFFECT_LOAD_ALL);
export const loadAllSuccess = createAction(
  SERVER_EFFECT_LOAD_ALL_SUCCESS,
  props<{ servers: Server[] }>()
);
export const loadAllFailure = createAction(
  SERVER_EFFECT_LOAD_ALL_FAILURE,
  props<{ error: Error }>()
);

export const loadSpecific = createAction(
  SERVER_EFFECT_LOAD_SPECIFIC,
  props<{ ipaddress: string; fullData: boolean }>()
);
export const loadSpecificSuccess = createAction(
  SERVER_EFFECT_LOAD_SPECIFIC_SUCCESS,
  props<{ server: Server }>()
);
export const loadSpecificFailure = createAction(
  SERVER_EFFECT_LOAD_SPECIFIC_FAILURE,
  props<{ error: Error }>()
);

export const saveServer = createAction(
  SERVER_EFFECT_SAVE_SERVER,
  props<{ server: Server }>()
);

export const saveServers = createAction(
  SERVER_EFFECT_SAVE_SERVERS,
  props<{ servers: Server[] }>()
);

export const removeServers = createAction(
  SERVER_EFFECT_REMOVE_SERVERS,
  props<{ servers: Server[] }>()
);

export const addServerFeature = createAction(
  SERVER_EFFECT_ADD_SERVER_FEATURE,
  props<{ serverFeature: ServerFeature }>()
);
export const addServerFeaturesSucces = createAction(
  SERVER_EFFECT_ADD_SERVER_FEATURE_SUCCESS
);

export const addServerFeatures = createAction(
  SERVER_EFFECT_ADD_SERVER_FEATURES,
  props<{ serverFeatures: ServerFeature[] }>()
);
export const addServerFeaturesSuccess = createAction(
  SERVER_EFFECT_ADD_SERVER_FEATURES_SUCCESS
);

export const removeServerFeature = createAction(
  SERVER_EFFECT_REMOVE_SERVER_FEATURE,
  props<{ serverFeature: ServerFeature }>()
);
export const removeServerFeatureSuccess = createAction(
  SERVER_EFFECT_REMOVE_SERVER_FEATURE_SUCCESS
);
