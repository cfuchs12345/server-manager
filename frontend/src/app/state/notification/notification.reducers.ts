import {  createReducer,  on } from '@ngrx/store';
import {
  addOne,
  addMany,
  updateOne,
  removeOne,
  upsertOne
} from 'src/app/state/notification/notification.actions';
import { EntityState, EntityAdapter, createEntityAdapter } from '@ngrx/entity';
import { Notifications } from 'src/app/services/notifications/types';

export type State = EntityState<Notifications>


export function selectIpAddress(a: Notifications): string {
  return a.ipaddress;
}


export const adapter: EntityAdapter<Notifications> =
  createEntityAdapter<Notifications>({
    selectId: selectIpAddress,
  });

export const initialStatusState: State = adapter.getInitialState({});

export const reducer  = createReducer(
  initialStatusState,

  on(addOne, (state, { notifications }) => {
    return adapter.addOne(notifications, state);
  }),

  on(addMany, (state, { notifications }) => {
    return adapter.addMany(notifications, state);
  }),

  on(removeOne, (state, { ipaddress }) => {
    return adapter.removeOne(ipaddress, state);
  }),

  on(updateOne, (state, { notifications }) => {
    return adapter.updateOne(notifications, state);
  }),

  on(upsertOne, (state, { notifications }) => {
    return adapter.upsertOne(notifications, state);
  })
);
