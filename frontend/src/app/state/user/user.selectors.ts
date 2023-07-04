import { createFeatureSelector, createSelector } from '@ngrx/store';
import { adapter, State, StateExist } from './user.reducers';

// get the selectors
const { selectAll } = adapter.getSelectors();

export const selectUsers = createFeatureSelector<State>('user');

export const selectUserExist = createFeatureSelector<StateExist>('userExist');

export const selectAllUsers = createSelector(selectUsers, selectAll);

export const selectUserByUserId = (user_id: string) =>
  createSelector(selectUsers, (state: State) => state.entities[user_id]);

export const selectUserExistByKey = () =>
  createSelector(selectUserExist, (state: StateExist) => state.entities["exist"]);

