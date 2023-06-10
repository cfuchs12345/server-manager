import { Component, ElementRef, OnInit, ViewChild } from '@angular/core';
import {
  FormControl,
  Validators,
} from '@angular/forms';
import { ErrorService, Source } from 'src/app/services/errors/error.service';
import { GeneralService } from 'src/app/services/general/general.service';

@Component({
  selector: 'app-config-im-export-modal',
  templateUrl: './config-im-export-modal.component.html',
  styleUrls: ['./config-im-export-modal.component.scss'],
})
export class ConfigImExportModalComponent implements OnInit {
  @ViewChild('fileSelector') fileSelector: ElementRef<HTMLElement> | undefined;

  fileUrl: string | undefined;
  uploadFileName: string = '';

  buttonTextGenerateDownloadLink = 'Generate File';
  buttonTextUploadFile = 'Upload Config';
  buttonTextSelectFile = 'Select File';

  passwordHint = 'Password/Key for config encryption';
  fileNameHint = '';
  passwordPlaceholder = 'Enter a password for the file';
  passwordLabel = 'Password/Key';

  fileNameLabel = 'Filename';

  password = new FormControl('', [
    Validators.required,
    Validators.minLength(6),
  ]);
  fileUpload = new FormControl('');
  fileName = new FormControl({ value: '', disabled: true });

  file: File | undefined = undefined;

  multiple: boolean = false;
  accept: string = '';

  constructor(
    private errorService: ErrorService,
    private generalService: GeneralService
  ) {
  }

  ngOnInit(): void {}

  onClickGenerateDownloadLink = () => {
    this.fileUrl = undefined;

    if( this.password == undefined || this.password.value == undefined) {
      return;
    }

    this.generalService.getConfig(this.password.value).subscribe({
      next: (res) => {
        const blob = new Blob([JSON.stringify(res, null, 2)], { type : 'application/json' });
        const objectUrl = URL.createObjectURL(blob);
        this.fileUrl = objectUrl;
      },
      error: (err: any) => {
        this.errorService.newError(Source.GeneralService, undefined, err);
      },
      complete: () => {
      },
  });
  };

  onSubmit = () => {
    if (
      this.file === undefined
    ) {
      return;
    }

    const fileReader = new FileReader();
    fileReader.readAsText(this.file, 'UTF-8');
    fileReader.onload = () => {
      if (fileReader.result !== undefined && fileReader.result !== null && this.password !== undefined && this.password.value !== null) {
        const json = JSON.parse(fileReader.result.toString());

        if( json !== undefined ) {
          this.generalService.uploadConfigFile(json, this.password.value);
        }
        else {
          this.errorService.newError(Source.GeneralService, undefined, "JSON config is invalid");
        }
      }
    };
    fileReader.onerror = (error) => {
      this.errorService.newError(Source.GeneralService, undefined, error);
    };


  };
  getPasswordMessage = () => {};

  onClickSelectFile() {
    if( this.fileSelector !== undefined) {
      this.fileSelector.nativeElement.click();
    }
  }

  onFileSelected(event: Event) {
    if (event === undefined || event === null || event.currentTarget === null) {
      return;
    }

    const element = event.currentTarget as HTMLInputElement;
    let fileList: FileList | null = element.files;

    if (fileList === null) {
      return;
    }

    if (fileList[0]) {
      this.file = fileList[0];
      this.fileName.setValue(this.file.name);
    } else {
      this.fileName.setValue('');
    }
  }
}
