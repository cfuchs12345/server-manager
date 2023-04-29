import { Component, OnInit, OnDestroy } from '@angular/core';
import { FormControl, Validators } from '@angular/forms';
import { Subscription } from 'rxjs';
import { GeneralService } from 'src/app/services/general/general.service';
import { DNSServer } from 'src/app/services/general/types';
import { ServerAddressType } from 'src/types/ServerAddress';

@Component({
  selector: 'app-configure-dns-modal',
  templateUrl: './configure-dns-modal.component.html',
  styleUrls: ['./configure-dns-modal.component.scss']
})
export class ConfigureDnsModalComponent implements OnInit, OnDestroy {
  buttonTextAddDNSServer: string = 'Add DNS Server';
  buttonTextDeleteDNSServers: string = 'Delete DNS Servers';

  ipplaceholder: string = 'xxx.xxx.xxx.xxx';
  ipAddressLabel: string = 'IP Address';
  ipaddressHint: string = 'Example: 192.168.178.111';
  ipaddress = new FormControl('', [
    Validators.required,
    Validators.pattern(ServerAddressType.IPV4),
  ]);

  portplaceholder: number = 53;
  portLabel: string = 'Port';
  portHint: string = 'Port is normally 53 for DNS servers';
  port = new FormControl('53', [Validators.required, Validators.pattern("\\d+")]);


  displayedColumns = ['delete', 'ipaddress', 'port'];

  dnsservers: DNSServer[]  = [];
  selectedDNSServers: string[] = [];
  dnsserversSubscription: Subscription | undefined = undefined;

  selectAll: boolean = false;



  constructor(private configService: GeneralService) {
  }

  ngOnInit(): void {
    this.dnsserversSubscription = this.configService.dnsServers.subscribe((dnsservers) => {
      if (dnsservers) {
        this.dnsservers = dnsservers;
      } else {
        // clear messages when empty message received
        this.dnsservers = [];
      }
    });

    this.configService.listDNSServers();
  }
  ngOnDestroy(): void {
    if(this.dnsserversSubscription) {
      this.dnsserversSubscription.unsubscribe();
    }
  }

  onClickSaveDNSServer = () => {
    if( this.ipaddress.value && this.port.value) {
      this.configService.saveDNSServer(new DNSServer(this.ipaddress.value, parseInt(this.port.value)));
    }
  }

  onClickDeleteDNSServers = () => {
    this.configService.deleteDNSServers(this.dnsservers.filter( (dnsServer) => this.isInList(dnsServer, this.selectedDNSServers)));
    this.selectedDNSServers = [];
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
