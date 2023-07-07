import {  createReducer,  on } from '@ngrx/store';
import { UserToken } from '../../services/users/types';
import {
  addOne,
  updateOne,
  removeOne,
  upsertOne
} from 'src/app/state/usertoken/usertoken.actions';
import { EntityAdapter, EntityState, createEntityAdapter } from '@ngrx/entity';

export type State = EntityState<UserToken>;


export function selectToken(): string {
  return "token";
}

export const adapter: EntityAdapter<UserToken> =
  createEntityAdapter<UserToken>({
    selectId: selectToken,
  });

export const initialUserState: State = adapter.getInitialState({});

export const reducer  = createReducer(
  initialUserState,

  on(addOne, (state, { usertoken }) => {
    return adapter.addOne(usertoken, state);
  }),


  on(removeOne, (state) => {
    return adapter.removeOne("token", state);
  }),

  on(updateOne, (state, { usertoken }) => {
    return adapter.updateOne(usertoken, state);
  }),

  on(upsertOne, (state, { usertoken }) => {
    return adapter.upsertOne(usertoken, state);
  })
);

