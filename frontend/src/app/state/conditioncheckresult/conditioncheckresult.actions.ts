import { createAction, props } from '@ngrx/store';
import { Update } from '@ngrx/entity';
import { ConditionCheckResult } from '../../services/servers/types';

export const CONDITIONCHECKRES_UPDATE_ACTION = '[ConditionCheckResult] Update';
export const CONDITIONCHECKRES_INSERT_ACTION = '[ConditionCheckResult] Insert';
export const CONDITIONCHECKRES_INSERT_MANY_ACTION = '[ConditionCheckResult] Insert Many';
export const CONDITIONCHECKRES_DELETE_ACTION = '[ConditionCheckResult] Delete';
export const CONDITIONCHECKRES_INSERT_OR_UPDATE_ACTION = '[ConditionCheckResult] Insert or Update';


export const updateOne = createAction(CONDITIONCHECKRES_UPDATE_ACTION, props<{ result: Update<ConditionCheckResult> }>());
export const addOne = createAction(CONDITIONCHECKRES_INSERT_ACTION, props<{ result: ConditionCheckResult }>());
export const addMany = createAction(CONDITIONCHECKRES_INSERT_MANY_ACTION, props<{ results: ConditionCheckResult[] }>());
export const removeOne = createAction(CONDITIONCHECKRES_DELETE_ACTION, props<{ key: string }>());


export const upsertOne = createAction(CONDITIONCHECKRES_INSERT_OR_UPDATE_ACTION, props<{ result: ConditionCheckResult }>());

