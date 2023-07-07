
import { createFeatureSelector, createSelector } from '@ngrx/store';
import {adapter, State}  from './plugin.reducers';



// get the selectors
const {
  selectAll,
} = adapter.getSelectors();


export const selectPluginState = createFeatureSelector<State>('plugin');

  export const selectAllPlugins = createSelector(
    selectPluginState,
    selectAll
  );


  export const selectPlugin = createSelector(
    selectPluginState,
    selectAll
  );



  export const  selectPluginById = (id: string) => createSelector(
    selectPluginState,
    (state: State) => state.entities[id]
  );
