
import { createFeatureSelector, createSelector } from '@ngrx/store';
import {adapter, State}  from '../reducers/conditioncheckresult.reducers';


// get the selectors
const {
  selectIds,
  selectEntities,
  selectAll,
  selectTotal,
} = adapter.getSelectors();


export const selectConditionCheckResultState = createFeatureSelector<State>('conditioncheckresult');

  export const selectAllConditionCheckResults = createSelector(
    selectConditionCheckResultState,
    selectAll
  );


  export const selectStatus = createSelector(
    selectConditionCheckResultState,
    selectAll
  );



  export const  selectConditionCheckResultByIpAddress = (ipaddress: string) => createSelector(
    selectConditionCheckResultState,
    (state: State) => state.entities[ipaddress]
  );
