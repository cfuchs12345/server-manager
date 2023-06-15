export class Notification {
  constructor(public id: string, public name: string, public ipaddress: string, public message: string, public notification_level: 'Info' | 'Warn' | 'Error' | 'Critical') {}
}
