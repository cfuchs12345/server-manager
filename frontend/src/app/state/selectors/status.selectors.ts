
import { createFeatureSelector, createSelector } from '@ngrx/store';
import {adapter, State}  from '../reducers/server-status.reducers';
import { Status } from 'src/app/services/servers/types';


// get the selectors
const {
  selectIds,
  selectEntities,
  selectAll,
  selectTotal,
} = adapter.getSelectors();


// select the array of users



export const selectStatusById = (ipaddress: string) =>
  createSelector(selectAll, (status) => status.find( s => s.ipaddress === ipaddress));


export const selectServerStatusState = createFeatureSelector<State>('status');

  export const selectAllServerStatus = createSelector(
    selectServerStatusState,
    selectAll
  );


  export const selectStatus = createSelector(
    selectServerStatusState,
    selectAll
  );



  export const  selectStatusByIpAddress = (ipaddress: string) => createSelector(
    selectServerStatusState,
    (state: State) => state.entities[ipaddress]
  );
