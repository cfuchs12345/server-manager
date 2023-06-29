import { isDevMode } from '@angular/core';
import {
  ActionReducerMap,
  MetaReducer
} from '@ngrx/store';

import * as serverStatus from './server-status.reducers';


export interface State {
  status: serverStatus.State,
}

export const reducers: ActionReducerMap<State> = {
  status: serverStatus.serverStatusReducer,
};


export const metaReducers: MetaReducer<State>[] = isDevMode() ? [] : [];



