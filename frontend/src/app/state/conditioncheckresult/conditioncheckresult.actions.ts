import { createAction, props } from '@ngrx/store';
import { Update } from '@ngrx/entity';
import { ConditionCheckResult } from '../../services/servers/types';

export const CONDITIONCHECKRES_UPDATE_ACTION = '[ConditionCheckResult] Update';
export const CONDITIONCHECKRES_INSERT_ACTION = '[ConditionCheckResult] Insert';
export const CONDITIONCHECKRES_INSERT_MANY_ACTION = '[ConditionCheckResult] Insert Many';
export const CONDITIONCHECKRES_DELETE_ACTION = '[ConditionCheckResult] Delete';
export const CONDITIONCHECKRES_INSERT_OR_UPDATE_ACTION = '[ConditionCheckResult] Insert or Update';

export const CONDITIONCHECKRES_EFFECT_LOAD_ALL = '[ConditionCheckResult] Effect: Load All';
export const CONDITIONCHECKRES_EFFECT_LOAD_ALL_SUCCESS = '[ConditionCheckResult] Effect: Load All Success';
export const CONDITIONCHECKRES_EFFECT_LOAD_ALL_FAILURE = '[ConditionCheckResult] Effect: Load All Failure';


export const updateOne = createAction(CONDITIONCHECKRES_UPDATE_ACTION, props<{ result: Update<ConditionCheckResult> }>());
export const addOne = createAction(CONDITIONCHECKRES_INSERT_ACTION, props<{ result: ConditionCheckResult }>());
export const addMany = createAction(CONDITIONCHECKRES_INSERT_MANY_ACTION, props<{ results: ConditionCheckResult[] }>());
export const removeOne = createAction(CONDITIONCHECKRES_DELETE_ACTION, props<{ key: string }>());


export const upsertOne = createAction(CONDITIONCHECKRES_INSERT_OR_UPDATE_ACTION, props<{ result: ConditionCheckResult }>());


// for effects
export const loadAll = createAction(CONDITIONCHECKRES_EFFECT_LOAD_ALL);
export const loadAllSuccess = createAction(CONDITIONCHECKRES_EFFECT_LOAD_ALL_SUCCESS, props<{results: ConditionCheckResult[]}>());
export const loadAllFailure = createAction(CONDITIONCHECKRES_EFFECT_LOAD_ALL_FAILURE, props<{error: Error}>());
