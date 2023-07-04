import { createFeatureSelector, createSelector } from '@ngrx/store';
import { State } from './usertoken.reducers';


export const selectTokens = createFeatureSelector<State>('usertoken');


export const selectToken = () =>
  createSelector(selectTokens, (state: State) => state.entities["token"]);
