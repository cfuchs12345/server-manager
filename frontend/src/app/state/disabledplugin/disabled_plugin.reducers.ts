import {  createReducer, on } from '@ngrx/store';
import {
  addOne,
  addMany,
  updateOne,
  removeOne,
  removeAll,
  upsertOne
} from 'src/app/state/disabledplugin/disabled_plugin.actions';
import { EntityState, EntityAdapter, createEntityAdapter } from '@ngrx/entity';


export type State = EntityState<string>


export function selectId(a: string): string {
  return a;
}



export const adapter: EntityAdapter<string> =
  createEntityAdapter<string>({
    selectId: selectId
  });

export const initialStatusState: State = adapter.getInitialState({});

export const reducer  = createReducer(
  initialStatusState,

  on(addOne, (state, { disabled_plugin }) => {
    return adapter.addOne(disabled_plugin, state);
  }),

  on(addMany, (state, { disabled_plugins }) => {
    return adapter.addMany(disabled_plugins, state);
  }),

  on(removeOne, (state, { disabled_plugin }) => {
    return adapter.removeOne(disabled_plugin, state);
  }),

  on(removeAll, (state) => {
    return adapter.removeAll(state);
  }),

  on(updateOne, (state, { disabled_plugin }) => {
    return adapter.updateOne(disabled_plugin, state);
  }),

  on(upsertOne, (state, { disabled_plugin }) => {
    return adapter.upsertOne(disabled_plugin, state);
  })

)

