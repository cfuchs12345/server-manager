import { createAction, props } from '@ngrx/store';
import { Update } from '@ngrx/entity';


export const DISABLED_PLUGIN_UPDATE_ACTION = '[Disabled Plugin] Update';
export const DISABLED_PLUGIN_INSERT_ACTION = '[Disabled Plugin] Insert';
export const DISABLED_PLUGIN_INSERT_MANY_ACTION = '[Disabled Plugin] Insert Many';
export const DISABLED_PLUGIN_DELETE_ACTION = '[Disabled Plugin] Delete';
export const DISABLED_PLUGIN_DELETE_ALL_ACTION = '[Disabled Plugin] Delete All';
export const DISABLED_PLUGIN_INSERT_OR_UPDATE_ACTION = '[Disabled Plugin] Insert or Update';


export const updateOne = createAction(DISABLED_PLUGIN_UPDATE_ACTION, props<{ disabled_plugin: Update<string> }>());
export const addOne = createAction(DISABLED_PLUGIN_INSERT_ACTION, props<{ disabled_plugin: string }>());
export const addMany = createAction(DISABLED_PLUGIN_INSERT_MANY_ACTION, props<{ disabled_plugins: string[] }>());
export const removeOne = createAction(DISABLED_PLUGIN_DELETE_ACTION, props<{ id: string }>());
export const removeAll = createAction(DISABLED_PLUGIN_DELETE_ALL_ACTION);
export const upsertOne = createAction(DISABLED_PLUGIN_INSERT_OR_UPDATE_ACTION, props<{ disabled_plugin: string }>());

