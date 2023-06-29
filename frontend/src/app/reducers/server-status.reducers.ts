import { createReducer, on } from '@ngrx/store';
import { Status } from '../services/servers/types';
import {
  statusInsert,
  statusUpdate,
  statusDelete,
  ServerStatus,
} from 'src/app/actions/server-status.action.types';
import { EntityState, EntityAdapter, createEntityAdapter } from '@ngrx/entity';

export interface State extends EntityState<ServerStatus> {}


export function selectIpAddress(a: ServerStatus): string {
  //In this case this would be optional since primary key is id
  return a.ipaddress;
}


export const adapter: EntityAdapter<ServerStatus> =
  createEntityAdapter<ServerStatus>({
    selectId: selectIpAddress,
  });

export const initialServerStatusState: State = adapter.getInitialState({});

export const serverStatusReducer  = createReducer(
  initialServerStatusState,

  on(statusInsert, (state, { status }) => {
    return adapter.addOne(status, state);
  }),

  on(statusDelete, (state, { ipaddress }) => {
    return adapter.removeOne(ipaddress, state);
  }),

  on(statusUpdate, (state, { status }) => {
    return adapter.updateOne(status, state);
  })
);

// get the selectors
const {
  selectIds,
  selectEntities,
  selectAll,
  selectTotal,
} = adapter.getSelectors();


// select the array of users
export const selectAllServerStatus = selectAll;
