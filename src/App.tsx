import { tauriBridge } from './bridge';
import Dashboard from './components/Dashboard';
import { useEffect, useRef, useState } from 'react';
import type { MacSentinelBridge, MonitoringSnapshot } from './types';
import { useMonitoring } from './useMonitoring';

interface AppProps {
	bridge?: MacSentinelBridge;
}

export default function App({ bridge = tauriBridge }: AppProps) {
	const monitoring = useMonitoring(bridge);
	const [notificationsDenied, setNotificationsDenied] = useState(false);
	const [samplingGapVisible, setSamplingGapVisible] = useState(false);
	const samplingGapTimer = useRef<number | null>(null);
	const recordedAtMs = monitoring.status === 'ready' ? monitoring.snapshot.recordedAtMs : null;
	const samplingGap = monitoring.status === 'ready' && monitoring.snapshot.samplingGap;

	useEffect(() => {
		if (!samplingGap) {
			return;
		}

		setSamplingGapVisible(true);
		if (samplingGapTimer.current !== null) {
			window.clearTimeout(samplingGapTimer.current);
		}
		samplingGapTimer.current = window.setTimeout(() => {
			setSamplingGapVisible(false);
			samplingGapTimer.current = null;
		}, 30_000);
	}, [recordedAtMs, samplingGap]);

	useEffect(() => () => {
		if (samplingGapTimer.current !== null) {
			window.clearTimeout(samplingGapTimer.current);
		}
	}, []);

	if (monitoring.status === 'loading') {
		return <main className="centered-state">Starting local monitoring…</main>;
	}

	if (monitoring.status === 'unavailable') {
		return (
			<main className="centered-state" role="alert">
				<h1>MacSentinel</h1>
				<p>{monitoring.message}</p>
			</main>
		);
	}

	let snapshot: MonitoringSnapshot = monitoring.snapshot;
	if (notificationsDenied) {
		snapshot = { ...snapshot, notificationsEnabled: false };
	}
	if (samplingGapVisible) {
		snapshot = { ...snapshot, samplingGap: true };
	}

	return (
		<>
			{import.meta.env.VITE_E2E_FIXTURES === 'true' && (
				<div className="e2e-fixtures" aria-label="E2E fixtures">
					<button
						type="button"
						data-testid="load-degraded-fixture"
						onClick={() => {
							setNotificationsDenied(true);
							void bridge.simulateStorageFailure();
						}}
					>Load degraded fixture</button>
					<button
						type="button"
						data-testid="load-sampling-gap-fixture"
						onClick={() => void bridge.simulateSamplingGap()}
					>Load sampling gap fixture</button>
				</div>
			)}
			<Dashboard snapshot={snapshot} loadHistory={bridge.getProcessHistory} />
		</>
	);
}
