import { isDevMode } from '@angular/core';
import {
  ActionReducer,
  ActionReducerMap,
  createFeatureSelector,
  createSelector,
  MetaReducer
} from '@ngrx/store';

import * as serverStatus from './server-status.reducers';

export interface State {
  serverStatus: serverStatus.State,
}

export const reducers: ActionReducerMap<State> = {
  serverStatus: serverStatus.serverStatusReducer,
};


export const metaReducers: MetaReducer<State>[] = isDevMode() ? [] : [];





export const selectServerStatusState = createFeatureSelector<serverStatus.State>('serverStatus');


export const selectAllServerStatus = createSelector(
  selectServerStatusState,
  serverStatus.selectAllServerStatus
);
