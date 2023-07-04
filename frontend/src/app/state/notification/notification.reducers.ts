import {  createReducer,  on } from '@ngrx/store';
import { Status } from '../../services/servers/types';
import {
  addOne,
  addMany,
  updateOne,
  removeOne,
  upsertOne
} from 'src/app/state/notification/notification.actions';
import { EntityState, EntityAdapter, createEntityAdapter } from '@ngrx/entity';
import { Notification } from 'src/app/services/notifications/types';

export type State = EntityState<Notification>


export function selectIpAddress(a: Notification): string {
  return a.ipaddress;
}


export const adapter: EntityAdapter<Notification> =
  createEntityAdapter<Notification>({
    selectId: selectIpAddress,
  });

export const initialStatusState: State = adapter.getInitialState({});

export const reducer  = createReducer(
  initialStatusState,

  on(addOne, (state, { notification }) => {
    return adapter.addOne(notification, state);
  }),

  on(addMany, (state, { notifications }) => {
    return adapter.addMany(notifications, state);
  }),

  on(removeOne, (state, { ipaddress }) => {
    return adapter.removeOne(ipaddress, state);
  }),

  on(updateOne, (state, { notification }) => {
    return adapter.updateOne(notification, state);
  }),

  on(upsertOne, (state, { notification }) => {
    return adapter.upsertOne(notification, state);
  })
);
