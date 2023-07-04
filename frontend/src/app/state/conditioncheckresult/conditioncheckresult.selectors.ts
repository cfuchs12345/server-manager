
import { createFeatureSelector, createSelector } from '@ngrx/store';
import {adapter, State}  from './conditioncheckresult.reducers';


// get the selectors
const {
  selectAll,
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



  export const  selectConditionCheckResultByKey = (key: string) => createSelector(
    selectConditionCheckResultState,
    (state: State) => state.entities[key]
  );
