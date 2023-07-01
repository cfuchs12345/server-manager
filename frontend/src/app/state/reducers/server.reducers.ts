import {  createReducer,  on } from '@ngrx/store';
import { Server } from '../../services/servers/types';
import {
  addOne,
  addMany,
  updateOne,
  removeOne,
  upsertOne
} from 'src/app/state/actions/server.action';
import { EntityState, EntityAdapter, createEntityAdapter } from '@ngrx/entity';
import  {ipToNumber} from 'src/app/shared/utils';


export type State = EntityState<Server>


export function selectIpAddress(a: Server): string {
  return a.ipaddress;
}


export function sortByIpAddress(a: Server, b: Server): number {
  return ipToNumber(a.ipaddress) - ipToNumber(b.ipaddress);
}

export const adapter: EntityAdapter<Server> =
  createEntityAdapter<Server>({
    selectId: selectIpAddress,
    sortComparer: sortByIpAddress
  });

export const initialStatusState: State = adapter.getInitialState({});

export const reducer  = createReducer(
  initialStatusState,

  on(addOne, (state, { server }) => {
    return adapter.addOne(server, state);
  }),

  on(addMany, (state, { servers }) => {
    return adapter.addMany(servers, state);
  }),

  on(removeOne, (state, { ipaddress }) => {
    return adapter.removeOne(ipaddress, state);
  }),

  on(updateOne, (state, { server }) => {
    return adapter.updateOne(server, state);
  }),

  on(upsertOne, (state, { server }) => {
    return adapter.upsertOne(server, state);
  })
);
