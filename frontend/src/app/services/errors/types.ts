import { Source } from './error.service';

export class Error {
  constructor(
    public source: Source,
    public ipaddress: string | undefined,
    public errorMessage: string,
    public lastOccurrance: Date,
    public count: number = 1
  ) {}

  increment = () => {
    this.count++;
  };

  setLastOccurrance(date: Date) {
    this.lastOccurrance = date;
  }
}
