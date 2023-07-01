export class Event {
  constructor(
    public object_type:
      | 'Status'
      | 'Server'
      | 'Plugin'
      | 'DisabledPlugins'
      | 'ConditionCheckResult'
      | 'Notification'
      | 'User',
    public event_type: 'Insert' | 'Update' | 'Delete',
    public key_name: string,
    public key: string,
    public value: string
  ) {}
}
