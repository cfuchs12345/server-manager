enum ServerAddressType {
    Hostname = "(([a-zA-Z0-9]|[a-zA-Z0-9][a-zA-Z0-9\\-]*[a-zA-Z0-9])\\.)+([A-Za-z]|[A-Za-z][A-Za-z0-9\\-]*[A-Za-z0-9])",
    IPV4 = "(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)",
    IPV6 = "(?:[A-F0-9]{1,4}:){7}[A-F0-9]{1,4}$",
    undefined = ''
}


interface IServerAddress {
    type: ServerAddressType,
    value: string
}

export { ServerAddressType as ServerAddressType };
export default IServerAddress;

