
import { createFeatureSelector, createSelector } from '@ngrx/store';
import {State}  from '../reducers/usertoken.reducers';




export const selectStatusState = createFeatureSelector<State>('usertoken');

  export const selectToken = createSelector(
    selectStatusState,
    (state) => state
  );
