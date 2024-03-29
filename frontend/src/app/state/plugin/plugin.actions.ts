import { createAction, props } from '@ngrx/store';
import { Update } from '@ngrx/entity';
import { Plugin } from 'src/app/services/plugins/types';


export const PLUGIN_UPDATE_ACTION = '[Plugin] Update';
export const PLUGIN_INSERT_ACTION = '[Plugin] Insert';
export const PLUGIN_INSERT_MANY_ACTION = '[Plugin] Insert Many';
export const PLUGIN_DELETE_ACTION = '[Plugin] Delete';
export const PLUGIN_DELETE_ALL_ACTION = '[Plugin] Delete All';
export const PLUGIN_INSERT_OR_UPDATE_ACTION = '[Plugin] Insert or Update';

export const PLUGIN_EFFECT_LOAD_ALL = '[Plugin] Effect: Load All';
export const PLUGIN_EFFECT_LOAD_ALL_SUCCESS = '[Plugin] Effect: Load All Success';
export const PLUGIN_EFFECT_LOAD_ALL_FAILURE = '[Plugin] Effect: Load All Failure';


export const updateOne = createAction(PLUGIN_UPDATE_ACTION, props<{ plugin: Update<Plugin> }>());
export const addOne = createAction(PLUGIN_INSERT_ACTION, props<{ plugin: Plugin }>());
export const addMany = createAction(PLUGIN_INSERT_MANY_ACTION, props<{ plugins: Plugin[] }>());
export const removeOne = createAction(PLUGIN_DELETE_ACTION, props<{ id: string }>());
export const removeAll = createAction(PLUGIN_DELETE_ALL_ACTION);
export const upsertOne = createAction(PLUGIN_INSERT_OR_UPDATE_ACTION, props<{ plugin: Plugin }>());


// for effects
export const loadAll = createAction(PLUGIN_EFFECT_LOAD_ALL);
export const loadAllSuccess = createAction(PLUGIN_EFFECT_LOAD_ALL_SUCCESS, props<{plugins: Plugin[]}>());
export const loadAllFailure = createAction(PLUGIN_EFFECT_LOAD_ALL_FAILURE, props<{error: Error}>());
