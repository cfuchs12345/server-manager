export class User {
  constructor(public user_id: string, public full_name: string, public email: string) {}
}

export class UserToken {
  constructor(public user_id: string, public token: string, public client_key: string) {}
}

export class UserPasswordHash {
  constructor(public user_id: string, public password_hash: string) {}
}

export class UserInitialPassword {
  constructor(public user_id: string, public password: string | undefined | null = undefined) {}
}
