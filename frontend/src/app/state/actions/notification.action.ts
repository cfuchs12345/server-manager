import { createAction, props } from '@ngrx/store';
import { Update } from '@ngrx/entity';
import { Notification } from 'src/app/services/notifications/types';


export const NOTIFICATION_UPDATE_ACTION = '[Notification] Update';
export const NOTIFICATION_INSERT_ACTION = '[Notification] Insert';
export const NOTIFICATION_INSERT_MANY_ACTION = '[Notification] Insert Many';
export const NOTIFICATION_DELETE_ACTION = '[Notification] Delete';
export const NOTIFICATION_INSERT_OR_UPDATE_ACTION = '[Notification] Insert or Update';


export const updateOne = createAction(NOTIFICATION_UPDATE_ACTION, props<{ notification: Update<Notification> }>());
export const addOne = createAction(NOTIFICATION_INSERT_ACTION, props<{ notification: Notification }>());
export const addMany = createAction(NOTIFICATION_INSERT_MANY_ACTION, props<{ notifications: Notification[] }>());
export const removeOne = createAction(NOTIFICATION_DELETE_ACTION, props<{ ipaddress: string }>());


export const upsertOne = createAction(NOTIFICATION_INSERT_OR_UPDATE_ACTION, props<{ notification: Notification }>());

