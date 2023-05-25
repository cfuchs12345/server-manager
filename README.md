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

## How the main screen looks like

![The main screen](main_screen.png)


## This is how a UPnP is showing it's data

![The Synology UPnP screen](synology_upnp.png)

## This is how the switch and information of a Tasmota device looks like

![The Tasmota screen](tasmota_switch.png)