import { TestBed } from '@angular/core/testing';

import { PluginService } from './plugin.service';
import { ErrorService } from '../errors/error.service';
import { LoggerTestingModule } from 'ngx-logger/testing';
import { HttpClientTestingModule } from '@angular/common/http/testing';


describe('PluginService', () => {
  let service: PluginService;
  let errorService: ErrorService;

  beforeEach(() => {
    TestBed.configureTestingModule({
      providers: [ErrorService],
      imports: [LoggerTestingModule, HttpClientTestingModule]
    });
    service = TestBed.inject(PluginService);
    errorService = TestBed.inject(ErrorService);

  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
