import { Injectable } from "@angular/core";
import { BehaviorSubject } from "rxjs";
import { Error } from "./types";

@Injectable()
export class ErrorService {
  private _errors = new BehaviorSubject<Map<string, Error>>( new Map());
  readonly errors = this._errors.asObservable();

  private dataStore: {
    errors: Map<string, Error>;
  } = {
    errors: new Map(),
  };


  constructor() { }

  newError(sourceName: string, ipaddress: string | undefined = undefined, errorMessage: string){
    this.publishError( new Date(), sourceName, ipaddress, errorMessage);
  }


  private publishError = (date: Date, sourceName: string, ipaddress: string | undefined, errorMessage: string) => {
    const key = sourceName + "|" + errorMessage;

    var error = this.dataStore.errors.get(key);
    if( !error) {
      error = new Error(sourceName, ipaddress, errorMessage, date, 1);
      this.dataStore.errors.set(key, error);
    }
    else {
      error.increment();
      error.setLastOccurrance(date);
    }
    this._errors.next(this.dataStore.errors);
  };


}
