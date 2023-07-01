import {  createReducer,  on } from '@ngrx/store';
import { ConditionCheckResult, Status } from '../../services/servers/types';
import {
  addOne,
  addMany,
  updateOne,
  removeOne,
  upsertOne
} from 'src/app/state/actions/conditioncheckresult.action';
import { EntityState, EntityAdapter, createEntityAdapter } from '@ngrx/entity';

export type State = EntityState<ConditionCheckResult>


export function selectIpAddress(a: ConditionCheckResult): string {
  return a.ipaddress;
}


export const adapter: EntityAdapter<ConditionCheckResult> =
  createEntityAdapter<ConditionCheckResult>({
    selectId: selectIpAddress,
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

  on(removeOne, (state, { ipaddress }) => {
    return adapter.removeOne(ipaddress, state);
  }),

  on(updateOne, (state, { result }) => {
    return adapter.updateOne(result, state);
  }),

  on(upsertOne, (state, { result }) => {
    return adapter.upsertOne(result, state);
  })
);
