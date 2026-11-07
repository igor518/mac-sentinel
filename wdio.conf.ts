import type { Options } from '@wdio/types';

export const config: Options.Testrunner = {
	runner: 'local',
	logLevel: 'error',
	specs: ['./e2e/**/*.e2e.ts'],
	maxInstances: 1,
	capabilities: [{
		browserName: 'tauri',
		'tauri:options': {
			application: './src-tauri/target/debug/macsentinel',
		},
	}],
	framework: 'mocha',
	reporters: ['spec'],
	services: [['@wdio/tauri-service', {
		driverProvider: 'embedded',
		appBinaryPath: './src-tauri/target/debug/macsentinel',
	}]],
	mochaOpts: {
		timeout: 60_000,
	},
};
