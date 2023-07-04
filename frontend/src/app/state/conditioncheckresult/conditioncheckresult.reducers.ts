import {  createReducer,  on } from '@ngrx/store';
import { ConditionCheckResult } from '../../services/servers/types';
import {
  addOne,
  addMany,
  updateOne,
  removeOne,
  upsertOne
} from 'src/app/state/conditioncheckresult/conditioncheckresult.actions';
import { EntityState, EntityAdapter, createEntityAdapter } from '@ngrx/entity';

export type State = EntityState<ConditionCheckResult>


export function selectIpAddressAndDataId(a: ConditionCheckResult): string {
  return a.ipaddress + "_" + a.data_id;
}


export const adapter: EntityAdapter<ConditionCheckResult> =
  createEntityAdapter<ConditionCheckResult>({
    selectId: selectIpAddressAndDataId,
  });

export const initialStatusState: State = adapter.getInitialState({});

export const reducer  = createReducer(
  initialStatusState,

  on(addOne, (state, { result }) => {
    return adapter.addOne(result, state);
  }),

  on(addMany, (state, { results }) => {
    return adapter.addMany(results, state);
  }),

  on(removeOne, (state, { key }) => {
    return adapter.removeOne(key, state);
  }),

  on(updateOne, (state, { result }) => {
    return adapter.updateOne(result, state);
  }),

  on(upsertOne, (state, { result }) => {
    return adapter.upsertOne(result, state);
  })
);
