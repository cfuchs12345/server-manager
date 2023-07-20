import { Pipe, PipeTransform } from '@angular/core';
import { Source } from '../services/errors/error.service';

@Pipe({
    name: 'sourceName',
    standalone: true,
})
export class ErrorSourceNamePipe implements PipeTransform {
  transform(value: Source): string {
    if (value === null) {
      return '';
    }

    return Source[value];
  }
}
