import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import type {
	MacSentinelBridge,
	MonitoringSnapshot,
	ProcessKey,
	Settings,
	TerminationResult,
} from './types';

const MONITORING_EVENT = 'monitoring://snapshot';

export const tauriBridge: MacSentinelBridge = {
	getCurrentSnapshot: () => invoke<MonitoringSnapshot>('get_current_snapshot'),
	subscribe: (listener) => {
		let active = true;
		let unlisten: (() => void) | undefined;

		void listen<MonitoringSnapshot>(MONITORING_EVENT, (event) => {
			if (active) {
				listener(event.payload);
			}
		}).then((stop) => {
			if (active) {
				unlisten = stop;
			} else {
				stop();
			}
		});

		return () => {
			active = false;
			unlisten?.();
		};
	},
	getSettings: () => invoke<Settings>('get_settings'),
	saveSettings: (settings: Settings) => invoke<Settings>('save_settings', { settings }),
	deleteHistory: () => invoke<void>('delete_history'),
	requestTermination: (key: ProcessKey) => (
		invoke<TerminationResult>('request_termination', { key })
	),
};
