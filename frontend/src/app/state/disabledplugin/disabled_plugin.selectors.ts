
import { createFeatureSelector, createSelector } from '@ngrx/store';
import {adapter, State}  from './disabled_plugin.reducers';



// get the selectors
const {
  selectIds,
  selectEntities,
  selectAll,
  selectTotal,
} = adapter.getSelectors();


export const selectDisabledPluginState = createFeatureSelector<State>('disabled_plugins');

  export const selectAllDisabledPlugins = createSelector(
    selectDisabledPluginState,
    selectAll
  );


  export const selectDisabledPlugin = createSelector(
    selectDisabledPluginState,
    selectAll
  );



  export const  selectDisabledPluginById = (id: string) => createSelector(
    selectDisabledPluginState,
    (state: State) => state.entities[id]
  );
