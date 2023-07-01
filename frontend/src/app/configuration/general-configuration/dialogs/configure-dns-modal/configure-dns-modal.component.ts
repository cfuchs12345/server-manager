import { Component, OnInit, OnDestroy } from '@angular/core';
import { FormControl, Validators } from '@angular/forms';
import { MatDialog } from '@angular/material/dialog';
import { Subscription } from 'rxjs';
import { GeneralService } from 'src/app/services/general/general.service';
import { DNSServer } from 'src/app/services/general/types';
import { ConfirmDialogComponent } from 'src/app/ui/confirm-dialog/confirm-dialog.component';
import { ServerAddressType } from 'src/types/ServerAddress';

@Component({
  selector: 'app-configure-dns-modal',
  templateUrl: './configure-dns-modal.component.html',
  styleUrls: ['./configure-dns-modal.component.scss']
})
export class ConfigureDnsModalComponent implements OnInit, OnDestroy {
  buttonTextAddDNSServer = 'Add DNS Server';
  buttonTextDeleteDNSServers = 'Delete DNS Servers';

  ipplaceholder = 'xxx.xxx.xxx.xxx';
  ipAddressLabel = 'IP Address';
  ipaddressHint = 'Example: 192.168.178.111';
  ipaddress = new FormControl('', [
    Validators.required,
    Validators.pattern(ServerAddressType.IPV4),
  ]);

  portplaceholder = 53;
  portLabel = 'Port';
  portHint = 'Port is normally 53 for DNS servers';
  port = new FormControl('53', [Validators.required, Validators.pattern("\\d+")]);


  displayedColumns = ['delete', 'ipaddress', 'port'];

  dnsservers: DNSServer[]  = [];
  systemDNSServers: DNSServer[]  = [];

  selectedDNSServers: string[] = [];
  dnsserversSubscription: Subscription | undefined = undefined;
  systemDnsserverSubscription: Subscription | undefined = undefined;

  selectAll = false;



  constructor(private configService: GeneralService,  private dialog: MatDialog) {
  }

  ngOnInit(): void {
    this.systemDnsserverSubscription = this.configService.listSystemDNSServers().subscribe( (systemDNSServers) => {
      if( systemDNSServers ) {
        this.systemDNSServers = systemDNSServers;
      }
      else {
        this.systemDNSServers = [];
      }
    });

    this.dnsserversSubscription = this.configService.listDNSServers().subscribe((dnsservers) => {
      if (dnsservers) {
        this.dnsservers = dnsservers;
      } else {
        // clear messages when empty message received
        this.dnsservers = [];
      }
    });

    setTimeout( () => {
      if( this.dnsservers.length === 0 && this.systemDNSServers.length !== 0) {
        this.showSystemDNSSuggestionDialog();
      }

    },1000);
  }


  ngOnDestroy(): void {
    if(this.dnsserversSubscription) {
      this.dnsserversSubscription.unsubscribe();
    }
  }

  onClickSaveDNSServer = () => {
    if( this.ipaddress.value && this.port.value) {
      this.configService.saveDNSServer(new DNSServer(this.ipaddress.value, parseInt(this.port.value)));

      this.configService.listDNSServers();
    }
  }

  onClickDeleteDNSServers = () => {
    this.configService.deleteDNSServers(this.dnsservers.filter( (dnsServer) => this.isInList(dnsServer, this.selectedDNSServers)));
    this.selectedDNSServers = [];

    this.configService.listDNSServers();
  }

  private showSystemDNSSuggestionDialog() {
    const dns_text = this.systemDNSServers.map((dnsserver) => " - " + dnsserver.ipaddress).join("<br>");
    const systemDNSDialog = this.dialog.open(ConfirmDialogComponent, {
      data: {
        title: 'Use found System DNS Servers',
        message: 'The following DNS servers where found on the server.<br/>Do you want to use thse as DNS servers?<br>' + dns_text,
      },
    });

    systemDNSDialog.afterClosed().subscribe((result) => {
      if (result === true) {
        for( const dnsserver of this.systemDNSServers) {
          this.configService.saveDNSServer(dnsserver);
        }

        this.systemDNSServers = [];
        setTimeout( this.configService.listDNSServers, 500);
      }
    });

  }

  private isInList(server: DNSServer, list: string[]) {
    return list.indexOf(server.ipaddress) >= 0;
  }


  getIPAddressErrorMessage = () => {
    if (this.ipaddress.hasError('required')) {
      return 'You must enter a value';
    }
    return this.ipaddress.hasError('pattern')
      ? 'The IP address format is not correct'
      : 'Unknown error';
  }

  getPortErrorMessage = () => {
    if (this.ipaddress.hasError('required')) {
      return 'You must enter a value';
    }
    return this.ipaddress.hasError('pattern')
      ? 'The port format is not correct'
      : 'Unknown error';
  }

  isSelected = (server: DNSServer): boolean => {
    return this.selectedDNSServers.indexOf(server.ipaddress) >= 0;
  }

  dnsServersSelected = (): number => {
    return this.selectedDNSServers.length;
  }

  onClickSelectServer = (server: DNSServer) => {
    if( this.selectedDNSServers && this.selectedDNSServers.indexOf(server.ipaddress) < 0) {
      this.selectedDNSServers.push(server.ipaddress);
    }
    else {
      this.selectedDNSServers = this.selectedDNSServers.filter( (ipaddress) => ipaddress !== server.ipaddress);
    }
  }

  onClickSelectAll = () => {
    this.selectAll = !this.selectAll;

    if( this.selectAll && this.dnsservers ) {
      this.selectedDNSServers = this.dnsservers.map( (server) => server.ipaddress);
    }
    else {
      this.selectedDNSServers = [];
    }
  }
}
