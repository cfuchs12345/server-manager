import { HttpHeaders } from '@angular/common/http';

export const defaultHeadersForJSON = (): HttpHeaders => {
  return new HttpHeaders({
    'Content-Type': 'application/json',
  });
};
