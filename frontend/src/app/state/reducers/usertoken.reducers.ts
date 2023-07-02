import {  createReducer,  on } from '@ngrx/store';
import { UserToken } from '../../services/users/types';
import {
  addOne,
  updateOne,
  removeOne,
  upsertOne
} from 'src/app/state/actions/usertoken.action';

export type State = UserToken | null;


export function selectUserId(a: UserToken): string {
  return a.user_id;
}


export const initialUserState: State = null;

export const reducer  = createReducer<State>(
  initialUserState,

  on(addOne, (state, { usertoken }): State =>  usertoken ),
  on(updateOne, (state, { usertoken }): State =>  usertoken ),
  on(upsertOne, (state, { usertoken }): State =>  usertoken ),

  on(removeOne, (): State => {
      return null;
  }),
);
