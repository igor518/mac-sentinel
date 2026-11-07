import { useState } from 'react';
import type { MacSentinelBridge, MonitoringSnapshot } from '../types';
import AlertCenter from './AlertCenter';
import ProcessExplorer from './ProcessExplorer';

interface DashboardProps {
	snapshot: MonitoringSnapshot;
	loadHistory: MacSentinelBridge['getProcessHistory'];
}

function metric(value: number | null, suffix = ''): string {
	return value === null ? 'Unavailable' : `${value}${suffix}`;
}

function formatCpu(value: number | null): string {
	return value === null ? 'Unavailable' : `${Math.round(value)}%`;
}

function formatMemory(bytes: number | null): string {
	return bytes === null ? 'Unavailable' : `${(bytes / 1_000_000_000).toFixed(1)} GB`;
}

export default function Dashboard({ snapshot, loadHistory }: DashboardProps) {
	const [summaryOpen, setSummaryOpen] = useState(false);
	const topProcesses = snapshot.processes.slice(0, 3);

	return (
		<main className="dashboard" data-testid="system-overview">
			<header className="dashboard__header">
				<div>
					<p className="eyebrow">System overview</p>
					<h1>MacSentinel</h1>
				</div>
				<div className="header-actions">
					<span className="status-pill">Local monitoring</span>
					<button
						type="button"
						data-testid="open-popover-fixture"
						aria-expanded={summaryOpen}
						onClick={() => setSummaryOpen((open) => !open)}
					>
						Summary
					</button>
				</div>
			</header>

			{summaryOpen && (
				<aside className="menu-summary" data-testid="menu-bar-summary" aria-label="Menu bar summary">
					<div><span>CPU</span><strong>{formatCpu(snapshot.systemCpuPercent)}</strong></div>
					<div><span>Memory</span><strong>{formatMemory(snapshot.usedMemoryBytes)}</strong></div>
					<div><span>Processes</span><strong>{metric(snapshot.processCount)}</strong></div>
					<ol>
						{topProcesses.map((process) => (
							<li key={`${process.key.pid}:${process.key.startedAt}`} data-testid="popover-top-process">
								<span>{process.name}</span>
								<strong>{process.cpuPercent === null ? 'Unavailable' : `${Math.round(process.cpuPercent)}%`}</strong>
							</li>
						))}
					</ol>
				</aside>
			)}

			{snapshot.storageStatus !== 'available' && (
				<div
					className="notice"
					role="status"
					aria-label="Storage unavailable"
					data-testid="storage-degraded"
				>
					History storage is {snapshot.storageStatus}. Live data remains independent.
				</div>
			)}

			{snapshot.notificationsEnabled === false && (
				<div className="notice" role="status" data-testid="notifications-denied">
					macOS notifications are disabled. Alerts remain available in MacSentinel.
				</div>
			)}

			{snapshot.samplingGap && (
				<div className="notice" role="status" data-testid="sampling-gap">
					Sampling resumed after a gap. <span data-testid="alert-continuity-reset">Sustained alert continuity was reset.</span>
				</div>
			)}

			<section className="metrics" aria-label="Current system metrics">
				<article className="metric-card">
					<span>CPU</span>
					<strong>{formatCpu(snapshot.systemCpuPercent)}</strong>
				</article>
				<article className="metric-card">
					<span>Memory</span>
					<strong>{formatMemory(snapshot.usedMemoryBytes)}</strong>
				</article>
				<article className="metric-card">
					<span>Processes</span>
					<strong data-testid="process-count">{metric(snapshot.processCount)}</strong>
				</article>
			</section>

			<p className="semantics" data-testid="cpu-semantics">
				System CPU spans the whole machine; process CPU may exceed 100% because 100% represents one logical core.
			</p>
			<AlertCenter alerts={snapshot.alerts} />
			{snapshot.processes.length === 0 ? (
				<p className="empty-state">Waiting for the first live process sample.</p>
			) : (
				<ProcessExplorer processes={snapshot.processes} loadHistory={loadHistory} />
			)}
		</main>
	);
}
