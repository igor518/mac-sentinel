import { useEffect, useState } from 'react';
import type { MacSentinelBridge, MonitoringSnapshot } from './types';

type MonitoringViewState =
	| { status: 'loading' }
	| { status: 'ready'; snapshot: MonitoringSnapshot }
	| { status: 'unavailable'; message: string };

export function useMonitoring(bridge: MacSentinelBridge): MonitoringViewState {
	const [state, setState] = useState<MonitoringViewState>({ status: 'loading' });

	useEffect(() => {
		let active = true;
		const unsubscribe = bridge.subscribe((snapshot) => {
			if (active) {
				setState({ status: 'ready', snapshot });
			}
		});

		void bridge.getCurrentSnapshot()
			.then((snapshot) => {
				if (active) {
					setState({ status: 'ready', snapshot });
				}
			})
			.catch(() => {
				if (active) {
					setState({
						status: 'unavailable',
						message: 'Live monitoring is currently unavailable.',
					});
				}
			});

		return () => {
			active = false;
			unsubscribe();
		};
	}, [bridge]);

	return state;
}
