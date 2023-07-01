
import { createFeatureSelector, createSelector } from '@ngrx/store';
import {adapter, State}  from '../reducers/server.reducers';


// get the selectors
const {
  selectAll,
} = adapter.getSelectors();



export const selectServerState = createFeatureSelector<State>('server');

  export const selectAllServers = createSelector(
    selectServerState,
    selectAll
  );

  export const selectAllServersWithFeatures = createSelector(
    selectAllServers,
    (servers) => servers.filter( (server) => server.features && server.features.length > 0)
  );


  export const selectServer = createSelector(
    selectServerState,
    selectAll
  );



  export const  selectServerByIpAddress = (ipaddress: string) => createSelector(
    selectServerState,
    (state: State) => state.entities[ipaddress]
  );
