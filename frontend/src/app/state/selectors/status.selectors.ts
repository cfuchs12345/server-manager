
import { createFeatureSelector, createSelector } from '@ngrx/store';
import {adapter, State}  from '../reducers/status.reducers';


// get the selectors
const {
  selectIds,
  selectEntities,
  selectAll,
  selectTotal,
} = adapter.getSelectors();


export const selectStatusState = createFeatureSelector<State>('status');

  export const selectAllServerStatus = createSelector(
    selectStatusState,
    selectAll
  );


  export const selectStatus = createSelector(
    selectStatusState,
    selectAll
  );



  export const  selectStatusByIpAddress = (ipaddress: string) => createSelector(
    selectStatusState,
    (state: State) => state.entities[ipaddress]
  );
