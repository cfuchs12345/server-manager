# Homelab Server Manager

## Description

The aim of this project is to give users a consolidated view about their servers and devices in their homelab.
In addition to monitoring functionality, it should be also possible to perform actions on these servers/devices like starting with Wak-on-Lan or restarting VMs of a Hypervisor.

All this should be extesible by simple plugins, which are currently defined as JSON files that include all necessary links and parameter definitions.
There can be also conditions defined when a action is possible to execute and which data is relevant for the condition. The conditions can be changed in script languges (currently LUA and RHAI).
In addition to these definitions, there can be templates linked which convert XML/JSON output into nice looking HTML output.

And since I'm lazy, I tried to automate everything as much as possible. For example, the servers, devices and their features can be auto-discovered by using ping, DNS queries, UPnP decvice discovery.
In addition to that, each plugin can give the manager the information how and where a feature can be detected (ie. by grabbing information from a status page).

Currently working Plugins/Features:

- OpnSense Firewall query
- Proxmox Hypervisor data query and VM/LXC control
- Wake-on-Lan
- Sleep-on-Lan
- UPnP device information query
- Tasmota (query data, power switch of power outlets)
- PiHole data query

The server manager can be executed as a docker container by using this docker-compose.yml:

    version: "3"
    services:
      server:
        image: docker.registry.lan:5000/afoxdocker/docker-server-manager
        container_name: server-manager
        environment:
           - USER_UID=1000
           - USER_GID=1000
        restart: always
        # if you want to use wake on lan actions, the network_mode has to be host
        # consequence -> if "network_mode: host" the port mapping doesn't work. You need to change the port of the internal server in the .env file in the config folder
        network_mode: host
        volumes:
          - ./config:/external_files
          - ./log:/var/log
          - /etc/timezone:/etc/timezone:ro
          - /etc/localtime:/etc/localtime:ro
          - /var/run/docker.sock:/var/run/docker.sock

Some notes:

- Passwords/credentials can be marked in the plugin so that they are automatically encrypted
- User passwords are not sent cleartext - even if the web server is running only via HTTP, since there is an internal AES-GCM encryption for sensitive data with one-time encryption key that always change
- Passwords are not stored as cleartext but using bcrypt hashes

My ToDo list:

- Docker plugin for Socket based installation on the same host
- Docker plugin for port based connection for supporting remote docker installations
- Extract more data for existing plugins
- Control UPnP devices with their exposed actions
- Store received information as time series and build graphs (Grafana-like)
- more Plugins (nearly everything that offers a REST, SOAP API is possible) CrowdSec, PfSense and many more is possible...
- maybe build an agent that can be installed on remote machines to get information from the OS or installations like Wireguard, that offer no direct API

## How the main screen looks like

![The main screen](main_screen.png)


## This is how a UPnP device is showing it's data

![The Synology UPnP screen](synology_upnp.png)

## This is how the switch and information of a Tasmota device looks like

![The Tasmota screen](tasmota_switch.png)