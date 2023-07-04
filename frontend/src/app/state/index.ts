import { inject, isDevMode } from '@angular/core';
import {
  ActionReducer,
  ActionReducerMap,
  MetaReducer
} from '@ngrx/store';

import * as status from './status/status.reducers';
import * as server from './server/server.reducers';
import * as plugin from './plugin/plugin.reducers';
import * as disabledPlugin from './disabledplugin/disabled_plugin.reducers';
import * as conditioncheckresult from './conditioncheckresult/conditioncheckresult.reducers';
import * as notification from './notification/notification.reducers';
import * as user from './user/user.reducers';
import * as usertoken from './usertoken/usertoken.reducers';
import { NGXLogger } from 'ngx-logger';
import { hydrationMetaReducer } from './hydration/hydration.reducers';

const LOCALSTORE_KEY = "state";

export interface State {
  disabled_plugins: disabledPlugin.State,
  status: status.State,
  server: server.State,
  plugin: plugin.State,
  conditioncheckresult: conditioncheckresult.State,
  notification: notification.State,
  user: user.State,
  usertoken: usertoken.State,
  userExist: user.StateExist
}

export const reducers: ActionReducerMap<State> = {
  disabled_plugins: disabledPlugin.reducer,
  status: status.reducer,
  server: server.reducer,
  plugin: plugin.reducer,
  conditioncheckresult: conditioncheckresult.reducer,
  notification: notification.reducer,
  user: user.reducer,
  usertoken: usertoken.reducer,
  userExist: user.reducerUserExist
};

/* eslint-disable @typescript-eslint/no-explicit-any */
export function debug(reducer: ActionReducer<any>): ActionReducer<any> {
  const logger = inject(NGXLogger);

  return function(state, action) {
    logger.trace("state and action", state, action);

    return reducer(state, action);
  };
}

export const metaReducers: MetaReducer<State>[] = isDevMode() ? [debug, hydrationMetaReducer] : [hydrationMetaReducer];


export const resetSavedState = () => {
  localStorage.removeItem(LOCALSTORE_KEY);
}

export const savedState = (state: State ) => {
  localStorage.setItem(LOCALSTORE_KEY, JSON.stringify(state));
}

export const getSavedState = (): State => {
  const json = localStorage.getItem(LOCALSTORE_KEY);

  return json ? JSON.parse(json) : {};
}


