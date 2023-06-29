import { createFeatureSelector, createReducer, createSelector, on } from '@ngrx/store';
import { Status } from '../../services/servers/types';
import {
  addOne,
  addMany,
  updateOne,
  removeOne,
  upsertOne
} from 'src/app/state/actions/server-status.action.types';
import { EntityState, EntityAdapter, createEntityAdapter } from '@ngrx/entity';

export interface State extends EntityState<Status> {}


export function selectIpAddress(a: Status): string {
  //In this case this would be optional since primary key is id
  return a.ipaddress;
}


export const adapter: EntityAdapter<Status> =
  createEntityAdapter<Status>({
    selectId: selectIpAddress,
  });

export const initialServerStatusState: State = adapter.getInitialState({});

export const serverStatusReducer  = createReducer(
  initialServerStatusState,

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
