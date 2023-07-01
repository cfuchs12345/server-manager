
import { createFeatureSelector, createSelector } from '@ngrx/store';
import {adapter, State}  from '../reducers/status.reducers';


// get the selectors
const {
  selectAll,
} = adapter.getSelectors();


export const selectStatusState = createFeatureSelector<State>('status');

  export const selectAllStatus = createSelector(
    selectStatusState,
    selectAll
  );


  export const  selectStatusByIpAddress = (ipaddress: string) => createSelector(
    selectStatusState,
    (state: State) => state.entities[ipaddress]
  );
