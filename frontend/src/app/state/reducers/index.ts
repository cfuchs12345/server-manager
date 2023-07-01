import { inject, isDevMode } from '@angular/core';
import {
  ActionReducer,
  ActionReducerMap,
  MetaReducer
} from '@ngrx/store';

import * as status from './status.reducers';
import * as server from './server.reducers';
import * as plugin from './plugin.reducers';
import * as disabledPlugin from './disabled_plugin.reducers';
import * as conditioncheckresult from './conditioncheckresult.reducers';
import * as notification from './notification.reducers';
import * as user from './user.reducers';
import { NGXLogger } from 'ngx-logger';


export interface State {
  disabled_plugins: disabledPlugin.State,
  status: status.State,
  server: server.State,
  plugin: plugin.State,
  conditioncheckresult: conditioncheckresult.State,
  notification: notification.State,
  user: user.State
}

export const reducers: ActionReducerMap<State> = {
  disabled_plugins: disabledPlugin.reducer,
  status: status.reducer,
  server: server.reducer,
  plugin: plugin.reducer,
  conditioncheckresult: conditioncheckresult.reducer,
  notification: notification.reducer,
  user: user.reducer
};


export function debug(reducer: ActionReducer<any>): ActionReducer<any> {
  const logger = inject(NGXLogger);

  return function(state, action) {
    logger.debug("state and action", state, action);

    return reducer(state, action);
  };
}

export const metaReducers: MetaReducer<State>[] = isDevMode() ? [debug] : [];



