import {  createReducer,  on } from '@ngrx/store';
import { User } from '../../services/users/types';
import {
  addOne,
  addMany,
  updateOne,
  removeOne,
  upsertOne,
  upsertOneExist,
  removeOneExist,
} from 'src/app/state/user/user.actions';
import { EntityState, EntityAdapter, createEntityAdapter } from '@ngrx/entity';

export type State = EntityState<User>
export type StateExist = EntityState<boolean>


export function selectUserId(a: User): string {
  return a.user_id;
}

export function selectUserExist(): string {
  return "exist";
}



export const adapter: EntityAdapter<User> =
  createEntityAdapter<User>({
    selectId: selectUserId,
  });

  export const adapterUserExist: EntityAdapter<boolean> =
  createEntityAdapter<boolean>({
    selectId: selectUserExist,
  });


export const initialUserState: State = adapter.getInitialState({});
export const initialUserExistState: StateExist = adapterUserExist.getInitialState({});

export const reducer  = createReducer(
  initialUserState,

  on(addOne, (state, { user }) => {
    return adapter.addOne(user, state);
  }),

  on(addMany, (state, { users }) => {
    return adapter.addMany(users, state);
  }),

  on(removeOne, (state, { user_id }) => {
    return adapter.removeOne(user_id, state);
  }),

  on(updateOne, (state, { user }) => {
    return adapter.updateOne(user, state);
  }),

  on(upsertOne, (state, { user }) => {
    return adapter.upsertOne(user, state);
  })
);

export const reducerUserExist  = createReducer(
  initialUserExistState,

  on(upsertOneExist, (state, { exist }) => {
    return adapterUserExist.upsertOne(exist, state);
  }),

  on(removeOneExist, (state) => {
    return adapterUserExist.removeOne("exist", state);
  }),
);
