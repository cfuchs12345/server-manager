export type ObjectType =
  | 'Status'
  | 'Server'
  | 'Plugin'
  | 'DisabledPlugins'
  | 'ConditionCheckResult'
  | 'Notification'
  | 'User';

export type EventType = 'Insert' | 'Update' | 'Delete' | 'Refresh';

export class Event {
  constructor(
    public object_type: ObjectType,
    public event_type: EventType,
    public key_name: string,
    public key: string,
    public value: string,
    public version: number
  ) {}
}
