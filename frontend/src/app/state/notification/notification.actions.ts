import { createAction, props } from '@ngrx/store';
import { Update } from '@ngrx/entity';
import { Notifications } from 'src/app/services/notifications/types';


export const NOTIFICATION_UPDATE_ACTION = '[Notification] Update';
export const NOTIFICATION_INSERT_ACTION = '[Notification] Insert';
export const NOTIFICATION_INSERT_MANY_ACTION = '[Notification] Insert Many';
export const NOTIFICATION_DELETE_ACTION = '[Notification] Delete';
export const NOTIFICATION_INSERT_OR_UPDATE_ACTION = '[Notification] Insert or Update';


export const updateOne = createAction(NOTIFICATION_UPDATE_ACTION, props<{ notifications: Update<Notifications> }>());
export const addOne = createAction(NOTIFICATION_INSERT_ACTION, props<{ notifications: Notifications }>());
export const addMany = createAction(NOTIFICATION_INSERT_MANY_ACTION, props<{ notifications: Notifications[] }>());
export const removeOne = createAction(NOTIFICATION_DELETE_ACTION, props<{ ipaddress: string }>());


export const upsertOne = createAction(NOTIFICATION_INSERT_OR_UPDATE_ACTION, props<{ notifications: Notifications }>());

