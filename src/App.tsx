import { tauriBridge } from './bridge';
import Dashboard from './components/Dashboard';
import type { MacSentinelBridge } from './types';
import { useMonitoring } from './useMonitoring';

interface AppProps {
	bridge?: MacSentinelBridge;
}

export default function App({ bridge = tauriBridge }: AppProps) {
	const monitoring = useMonitoring(bridge);

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

	return <Dashboard snapshot={monitoring.snapshot} />;
}
