
import { createFeatureSelector, createSelector } from '@ngrx/store';
import {adapter, State}  from './notification.reducers';


// get the selectors
const {
  selectIds,
  selectEntities,
  selectAll,
  selectTotal,
} = adapter.getSelectors();


export const selectNotificationState = createFeatureSelector<State>('notification');

  export const selectAllNotification = createSelector(
    selectNotificationState,
    selectAll
  );


  export const selectNotification = createSelector(
    selectNotificationState,
    selectAll
  );



  export const  selectotificationByIpAddress = (ipaddress: string) => createSelector(
    selectNotificationState,
    (state: State) => state.entities[ipaddress]
  );
