import {  createReducer,  on } from '@ngrx/store';
import { Status } from '../../services/servers/types';
import {
  addOne,
  addMany,
  updateOne,
  removeOne,
  upsertOne
} from 'src/app/state/actions/status.action';
import { EntityState, EntityAdapter, createEntityAdapter } from '@ngrx/entity';

export type State = EntityState<Status>


export function selectIpAddress(a: Status): string {
  return a.ipaddress;
}


export const adapter: EntityAdapter<Status> =
  createEntityAdapter<Status>({
    selectId: selectIpAddress,
  });

export const initialStatusState: State = adapter.getInitialState({});

export const reducer  = createReducer(
  initialStatusState,

  on(addOne, (state, { status }) => {
    return adapter.addOne(status, state);
  }),

  on(addMany, (state, { status }) => {
    return adapter.addMany(status, state);
  }),

  on(removeOne, (state, { ipaddress }) => {
    return adapter.removeOne(ipaddress, state);
  }),

  on(updateOne, (state, { status }) => {
    return adapter.updateOne(status, state);
  }),

  on(upsertOne, (state, { status }) => {
    return adapter.upsertOne(status, state);
  })
);
