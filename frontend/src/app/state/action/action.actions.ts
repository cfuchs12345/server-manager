import { createAction, props } from '@ngrx/store';


export const EXECUTE_ACTION = '[Action] Execute';
export const EXECUTE_ACTION_SUCCESS = '[Action] Execute Success';
export const EXECUTE_ACTION_FAILURE = '[Action] Execute Failure';


// for effects
export const executeAction = createAction(EXECUTE_ACTION, props<{feature_id: string, action_id: string, ipaddress: string, action_params: string | undefined}>());
export const executeActionSuccess = createAction(EXECUTE_ACTION_SUCCESS);
export const executeActionFailure = createAction(EXECUTE_ACTION_FAILURE, props<{error: Error}>());
