import { createFeatureSelector, createSelector } from '@ngrx/store';
import { State, adapter } from './usertoken.reducers';

const { selectAll } = adapter.getSelectors();

export const selectTokens = createFeatureSelector<State>('usertoken');

export const selectAllTokens = createSelector(selectTokens, selectAll);

export const selectToken = () =>
  createSelector(selectTokens, (state: State) => state.entities["token"]);
