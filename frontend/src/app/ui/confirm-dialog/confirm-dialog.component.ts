import { Component, inject } from '@angular/core';
import { MAT_DIALOG_DATA, MatDialogRef, MatDialogModule } from '@angular/material/dialog';
import { DomSanitizer, SafeHtml } from '@angular/platform-browser';
import { MatButtonModule } from '@angular/material/button';

@Component({
    selector: 'app-confirm-dialog',
    templateUrl: './confirm-dialog.component.html',
    styleUrls: ['./confirm-dialog.component.scss'],
    standalone: true,
    imports: [MatDialogModule, MatButtonModule],
})
export class ConfirmDialogComponent {
  public dialogRef = inject(MatDialogRef<ConfirmDialogComponent>);
  private sanitizer = inject(DomSanitizer);
  public data = inject(MAT_DIALOG_DATA);

  buttonTextConfirm = 'Confirm';
  buttonTextCancel = 'Cancel';

  get_content(): SafeHtml {
    return this.sanitizer.bypassSecurityTrustHtml(this.data.message);
  }
}
