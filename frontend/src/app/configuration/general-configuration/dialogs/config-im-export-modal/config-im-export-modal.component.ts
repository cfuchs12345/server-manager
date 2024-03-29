import {
  Component,
  ElementRef,
  OnDestroy,
  ViewChild,
  inject,
} from '@angular/core';
import { FormControl, Validators, FormsModule, ReactiveFormsModule } from '@angular/forms';
import { ErrorService, Source } from 'src/app/services/errors/error.service';
import { GeneralService } from 'src/app/services/general/general.service';
import { SubscriptionHandler } from 'src/app/shared/subscriptionHandler';
import { MatButtonModule } from '@angular/material/button';
import { NgIf } from '@angular/common';
import { MatInputModule } from '@angular/material/input';
import { MatFormFieldModule } from '@angular/material/form-field';
import { FlexModule } from '@angular/flex-layout/flex';

@Component({
    selector: 'app-config-im-export-modal',
    templateUrl: './config-im-export-modal.component.html',
    styleUrls: ['./config-im-export-modal.component.scss'],
    standalone: true,
    imports: [
        FlexModule,
        MatFormFieldModule,
        MatInputModule,
        FormsModule,
        ReactiveFormsModule,
        NgIf,
        MatButtonModule,
    ],
})
export class ConfigImExportModalComponent implements OnDestroy {
  @ViewChild('fileSelector') fileSelector: ElementRef<HTMLElement> | undefined;

  private errorService = inject(ErrorService);
  private generalService = inject(GeneralService);
  private subscriptionHandler = new SubscriptionHandler(this);

  fileUrl: string | undefined;
  uploadFileName = '';

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

  multiple = false;
  accept = '';


  ngOnDestroy(): void {
    this.subscriptionHandler.onDestroy();
  }

  onClickGenerateDownloadLink = () => {
    this.fileUrl = undefined;

    if (!this.password || !this.password.value) {
      return;
    }

    this.subscriptionHandler.subscription = this.generalService
      .getConfig(this.password.value)
      .subscribe({
        next: (res) => {
          const blob = new Blob([JSON.stringify(res, null, 2)], {
            type: 'application/json',
          });
          const objectUrl = URL.createObjectURL(blob);
          this.fileUrl = objectUrl;
        },
        error: (err) => {
          this.errorService.newError(Source.GeneralService, undefined, err);
        },
      });
  };

  onSubmit = () => {
    if (!this.file) {
      return;
    }

    const fileReader = new FileReader();
    fileReader.readAsText(this.file, 'UTF-8');
    fileReader.onload = () => {
      if (fileReader.result && this.password && this.password.value) {
        const json = JSON.parse(fileReader.result.toString());

        if (json) {
          this.generalService.uploadConfigFile(json, this.password.value);
        } else {
          this.errorService.newError(
            Source.GeneralService,
            undefined,
            'JSON config is invalid'
          );
        }
      }
    };
    fileReader.onerror = (error) => {
      this.errorService.newError(Source.GeneralService, undefined, error);
    };
  };

  getPasswordMessage = (): string => {
    return "Password Error";
  };

  onClickSelectFile() {
    if (this.fileSelector) {
      this.fileSelector.nativeElement.click();
    }
  }

  onFileSelected(event: Event) {
    if (!event || !event.currentTarget) {
      return;
    }
    const element = event.currentTarget as HTMLInputElement;

    if (element.files) {
      const fileList: FileList = element.files;

      if (fileList[0]) {
        this.file = fileList[0];
        this.fileName.setValue(this.file.name);
      } else {
        this.fileName.setValue('');
      }
    }
  }
}
