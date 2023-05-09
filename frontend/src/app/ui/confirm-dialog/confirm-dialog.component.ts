import { Component, Inject, OnInit } from '@angular/core';
import { MAT_DIALOG_DATA, MatDialogRef } from '@angular/material/dialog';
import { DomSanitizer, SafeHtml } from '@angular/platform-browser';

@Component({
  selector: 'app-confirm-dialog',
  templateUrl: './confirm-dialog.component.html',
  styleUrls: ['./confirm-dialog.component.scss']
})
export class ConfirmDialogComponent {
  buttonTextConfirm: string = 'Confirm';
  buttonTextCancel: string = 'Cancel';

  constructor(public dialogRef: MatDialogRef<ConfirmDialogComponent>,
              private sanitizer: DomSanitizer,
              @Inject(MAT_DIALOG_DATA) public data: any) { }

  get_content() : SafeHtml {
    console.log(this.data.message);
    return this.sanitizer.bypassSecurityTrustHtml(this.data.message);
  }
}
