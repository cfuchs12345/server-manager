export class Notification {
  constructor(public id: string, public name: string, public message: string, public notification_level: 'Info' | 'Warn' | 'Error' | 'Critical') {}
}


export class Notifications {
  constructor(public ipaddress: string, public list: Notification[] = []) {}
}
