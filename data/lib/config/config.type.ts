export type Config = {
	stage?: "test" | "stage" | "dev" | "prod";
	port: number;
	name: string;
	f1BaseUrl: string;
	allowOrigin: string;
	globalPrefix: string;
	defaultVersion: number;
	cacheTtl: number;
	cacheLimit: number;
	throttleTtl: number;
	throttleLimit: number;
	databasePath: string | ":memory:";
	databaseMode: "readwrite" | "readonly";
	logLevel: "trace" | "debug" | "info" | "warn" | "error";
	logRequests: boolean;
	logResponses: boolean;
};
