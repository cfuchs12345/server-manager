import { Injectable } from '@angular/core';
import {
  HttpClient,
  HttpErrorResponse,
} from '@angular/common/http';
import { BehaviorSubject } from 'rxjs';
import { defaultHeadersForJSON } from '../common';
import { DataResult } from './types';

import {
  Server,
  Param,
  ServerAction,
  Feature,
  ServerFeature,
} from './types';
import { ErrorService } from '../errors/error.service';

@Injectable({
  providedIn: 'root',
})
export class ServerService {
  private _servers = new BehaviorSubject<Server[]>([]);
  private dataStore: {
    servers: Server[];
  } = {
    servers: [],
  };

  readonly servers = this._servers.asObservable();

  constructor(private http: HttpClient, private errorService: ErrorService) {}

  deleteServers = (servers: Server[]) => {
    var isLast = false;

    for (const [i, server] of servers.entries()) {
      this.http
        .delete<any>('/backend/servers/' + server.ipaddress, {
          headers: defaultHeadersForJSON(),
        })
        .subscribe({
          next: (res) => {
            const indexToDelete = this.dataStore.servers.findIndex(s => s.ipaddress === server.ipaddress);
            this.dataStore.servers.splice(indexToDelete, 1);
          },
          error: (err: any) => {
            this.errorService.newError("Server-Service", server.ipaddress, err.message);
          },
          complete: () => {
            if (servers[servers.length -1].ipaddress === server.ipaddress) {
              setTimeout(this.publishServers, 500);
            }
          },
        });
    }
  };

  listServers = () => {
    this.http.get<Server[]>('/backend/servers').subscribe({
      next: (servers) => {
        this.dataStore.servers = servers;
      },
      error: (err: any) => {
        this.errorService.newError("Server-Service", undefined, err.message);
      },
      complete: () => {
        setTimeout(this.publishServers, 500);
      },
    });
  };

  saveServers = (servers: Server[]) => {
    for (const [i, server] of servers.entries()) {
      const body = JSON.stringify(server);

      this.http
        .post<any>('/backend/servers', body, {
          headers: defaultHeadersForJSON(),
        })
        .subscribe({
          next: (res) => {
            this.dataStore.servers.push(server);
          },
          error: (err: any) => {
            this.errorService.newError("Server-Service", server.ipaddress, err.message);
          },
          complete: () => {
            if ( i === servers.length -1) {
              setTimeout(this.publishServers, 500);
            }
          },
        });
    }
  };


  updateServerFeatures = (featuresToSet: ServerFeature[]) => {
    var featuresToSetMap: Map<string, Feature[]> = new Map();
    featuresToSet.forEach((server_feature) => {
      featuresToSetMap.set(server_feature.ipaddress, server_feature.features);
    });

    const serversToUpdate = [];

    for (var i = 0; i < this.dataStore.servers.length; i++) {
      var server = this.dataStore.servers[i];

      var featuresToSetForServer = featuresToSetMap.get(server.ipaddress);
      if (featuresToSetForServer === undefined) {
        // no feature for current server from iteration -> continue with next
        continue;
      }
      let newFeatureList = this.updateOrAddFeature(
        featuresToSetForServer,
        server
      );
      newFeatureList = this.removeFeaturesNoLongerInList(
        featuresToSetForServer,
        server
      );
      serversToUpdate.push(newFeatureList);
    }

    this.updateServers(serversToUpdate);
  };


  updateServers = (servers: Server[]) => {
    for (const server of servers) {
      const body = JSON.stringify(server);

      this.http
        .put<any>('/backend/servers/' + server.ipaddress, body, {
          headers: defaultHeadersForJSON(),
        })
        .subscribe({
          next: (res) => {
          },
          error: (err: any) => {
            this.errorService.newError("Server-Service", server.ipaddress, err.message);
          },
          complete: () => {
            setTimeout(this.publishServers, 500);
          },
        });
    }
  };

  private publishServers = () => {
    this.dataStore.servers.sort( this.compareServers )
    this._servers.next(this.dataStore.servers.slice(0, this.dataStore.servers.length));
  };


  private compareServers = (a: Server, b: Server): number => {
    const numA = Number(
      a.ipaddress.split('.')
        .map((num, idx) => parseInt(num) * Math.pow(2, (3 - idx) * 8))
        .reduce((a, v) => ((a += v), a), 0)
    );
    const numB = Number(
      b.ipaddress.split('.')
        .map((num, idx) => parseInt(num) * Math.pow(2, (3 - idx) * 8))
        .reduce((a, v) => ((a += v), a), 0)
    );
    return numA - numB;
  }


  private updateOrAddFeature = (
    foundFeature: Feature[],
    server: Server
  ): Server => {
    for (var feature of foundFeature) {
      var existing = server.features.find((f) => f.id === feature.id);

      if (!existing) {
        server.features.push(
          new Feature(
            feature.id,
            feature.name,
            feature.params,
            feature.credentials
          )
        );
      } else {
        existing.name = feature.name;
        existing.params = feature.params;
        existing.credentials = feature.credentials;
      }
    }
    return server;
  };

  private removeFeaturesNoLongerInList = (
    featuresToSetForServer: Feature[],
    server: Server
  ): Server => {
    server.features = server.features.filter((existingFeature) =>
      this.isInList(existingFeature, featuresToSetForServer)
    );

    return server;
  };

  private isInList = (
    featureToCheck: Feature,
    featureListToCheck: Feature[]
  ): boolean => {
    return (
      featureListToCheck.find((feature) => feature.id == featureToCheck.id) !== undefined
    );
  };
}
