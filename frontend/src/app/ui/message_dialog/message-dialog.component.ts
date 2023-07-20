import { Component, inject } from '@angular/core';
import { MAT_DIALOG_DATA, MatDialogRef, MatDialogModule } from '@angular/material/dialog';
import { DomSanitizer, SafeHtml } from '@angular/platform-browser';
import { ConfirmDialogComponent } from '../confirm-dialog/confirm-dialog.component';

@Component({
    selector: 'app-message-dialog',
    templateUrl: './message-dialog.component.html',
    styleUrls: ['./message-dialog.component.scss'],
    standalone: true,
    imports: [MatDialogModule],
})
export class MessageDialogComponent {
  public dialogRef = inject(MatDialogRef<ConfirmDialogComponent>);
  private sanitizer = inject(DomSanitizer);
  public data = inject(MAT_DIALOG_DATA);

  get_content(): SafeHtml {
    return this.sanitizer.bypassSecurityTrustHtml(this.data.message);
  }
}
