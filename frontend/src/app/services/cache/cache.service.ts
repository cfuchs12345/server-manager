import { Injectable } from "@angular/core";

@Injectable()
export class CacheService {
  private cache: Map<string, any> = new Map();

  constructor() {
    console.log("Cache constructor called");
  }

  public get<Type>(key: string): Type | undefined {
    const found:Type = this.cache.get(key);

      if( found ) {
        return this.getCopy(found);
      }
      else {
        return undefined;
      }
  }

  public set<Type>(key: string, value: Type) {
    return this.cache.set(key, this.getCopy(value));
  }

  private getCopy<Type>(value: Type): Type {
    const copy =  Object.assign({}, value);

    return copy;
  }
}
