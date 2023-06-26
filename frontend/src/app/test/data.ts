import { Server, Feature, Param, Credential } from "../services/servers/types";

export const getTestServer = (): Server => {
  return new Server("192.168.178.1", "test", "test.lan",  [ getFeature() ], false);
}

export const getFeature = (): Feature => {
  return new Feature("test", "Test Feature", [getParam()], [getCredential()]);
}

export const getParam = (): Param => {
  return new Param("Test", "value");
}

export const getCredential = (): Credential => {
  return new Credential("Password", "Test", false);
}
