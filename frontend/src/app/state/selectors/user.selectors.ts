
import { createFeatureSelector, createSelector } from '@ngrx/store';
import {adapter, State}  from '../reducers/user.reducers';


// get the selectors
const {
  selectAll,
} = adapter.getSelectors();


export const selectStatusState = createFeatureSelector<State>('user');

  export const selectAllUsers = createSelector(
    selectStatusState,
    selectAll
  );



  export const  selectUserByUserId = (user_id: string) => createSelector(
    selectStatusState,
    (state: State) => state.entities[user_id]
  );
