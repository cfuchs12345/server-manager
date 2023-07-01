import {  createReducer, on } from '@ngrx/store';
import {
  addOne,
  addMany,
  updateOne,
  removeOne,
  removeAll,
  upsertOne
} from 'src/app/state/actions/plugin.action';
import { EntityState, EntityAdapter, createEntityAdapter } from '@ngrx/entity';
import { Plugin } from 'src/app/services/plugins/types';


export type State = EntityState<Plugin>


export function selectId(a: Plugin): string {
  return a.id;
}


export function sortByName(a: Plugin, b: Plugin): number {
  return a.name.localeCompare(b.name);
}

export const adapter: EntityAdapter<Plugin> =
  createEntityAdapter<Plugin>({
    selectId: selectId,
    sortComparer: sortByName
  });

export const initialStatusState: State = adapter.getInitialState({});

export const reducer  = createReducer(
  initialStatusState,

  on(addOne, (state, { plugin }) => {
    return adapter.addOne(plugin, state);
  }),

  on(addMany, (state, { plugins }) => {
    return adapter.addMany(plugins, state);
  }),

  on(removeOne, (state, { id }) => {
    return adapter.removeOne(id, state);
  }),

  on(removeAll, (state) => {
    return adapter.removeAll(state);
  }),

  on(updateOne, (state, { plugin }) => {
    return adapter.updateOne(plugin, state);
  }),

  on(upsertOne, (state, { plugin }) => {
    return adapter.upsertOne(plugin, state);
  })
);
