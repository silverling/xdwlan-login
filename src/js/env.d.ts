declare global {
  namespace NodeJS {
    interface ProcessEnv {
      XDWLAN_LOGIN_URL: string;
      XDWLAN_USERNAME: string;
      XDWLAN_PASSWORD: string;
      XDWLAN_DOMAIN: string;
      NODE_ENV: "development" | "production";
    }
  }
}

export {};
