import { createFeatureSelector, createSelector } from '@ngrx/store';
import { adapter, State } from './user.reducers';

// get the selectors
const { selectAll } = adapter.getSelectors();

export const selectUsers = createFeatureSelector<State>('user');


export const selectAllUsers = createSelector(selectUsers, selectAll);

export const selectUserByUserId = (user_id: string) =>
  createSelector(selectUsers, (state: State) => state.entities[user_id]);
