import { useEffect, useMemo, useRef, useState } from 'react';
import type { ProcessKey, ProcessSample, ProcessSnapshot } from '../types';

interface ProcessExplorerProps {
	processes: ProcessSnapshot[];
	loadHistory: (key: ProcessKey, fromMs: number, toMs: number) => Promise<ProcessSample[]>;
}

type SortKey = 'cpu' | 'memory' | 'name' | 'pid';

function formatBytes(bytes: number | null): string {
	if (bytes === null) {
		return 'Unavailable';
	}

	return `${(bytes / 1_000_000_000).toFixed(1)} GB`;
}

function formatCpu(cpu: number | null): string {
	return cpu === null ? 'Unavailable' : `${Math.round(cpu)}%`;
}

function compareProcesses(left: ProcessSnapshot, right: ProcessSnapshot, sort: SortKey): number {
	switch (sort) {
		case 'memory':
			return right.memoryBytes - left.memoryBytes;
		case 'name':
			return left.name.localeCompare(right.name);
		case 'pid':
			return left.key.pid - right.key.pid;
		case 'cpu':
		default:
			return (right.cpuPercent ?? -1) - (left.cpuPercent ?? -1);
	}
}

type HistoryState =
	| { status: 'idle' | 'loading' }
	| { status: 'ready'; samples: ProcessSample[] }
	| { status: 'unavailable' };

export default function ProcessExplorer({ processes, loadHistory }: ProcessExplorerProps) {
	const [query, setQuery] = useState('');
	const [sort, setSort] = useState<SortKey>('cpu');
	const [selected, setSelected] = useState<ProcessSnapshot | null>(null);
	const closeButtonRef = useRef<HTMLButtonElement>(null);
	const [history, setHistory] = useState<HistoryState>({ status: 'idle' });

	useEffect(() => {
		if (!selected) {
			return undefined;
		}

		const previouslyFocused = document.activeElement as HTMLElement | null;
		const previousOverflow = document.body.style.overflow;
		const handleKeyDown = (event: KeyboardEvent) => {
			if (event.key === 'Escape') {
				setSelected(null);
			}

			if (event.key === 'Tab') {
				event.preventDefault();
				closeButtonRef.current?.focus();
			}
		};

		document.body.style.overflow = 'hidden';
		document.addEventListener('keydown', handleKeyDown);
		closeButtonRef.current?.focus();

		return () => {
			document.body.style.overflow = previousOverflow;
			document.removeEventListener('keydown', handleKeyDown);
			previouslyFocused?.focus();
		};
	}, [selected]);

	useEffect(() => {
		if (!selected) {
			setHistory({ status: 'idle' });
			return undefined;
		}

		let active = true;
		const toMs = Date.now();
		setHistory({ status: 'loading' });
		void loadHistory(selected.key, toMs - 15 * 60 * 1_000, toMs)
			.then((samples) => {
				if (active) {
					setHistory({ status: 'ready', samples });
				}
			})
			.catch(() => {
				if (active) {
					setHistory({ status: 'unavailable' });
				}
			});

		return () => {
			active = false;
		};
	}, [loadHistory, selected]);
	const visible = useMemo(() => {
		const normalized = query.trim().toLocaleLowerCase();

		const filtered = processes
			.filter((process) => (
				normalized.length === 0
				|| process.name.toLocaleLowerCase().includes(normalized)
				|| String(process.key.pid).includes(normalized)
			));
		filtered.sort((left, right) => compareProcesses(left, right, sort));

		return filtered;
	}, [processes, query, sort]);

	return (
		<section className="process-explorer">
			<div className="section-heading">
				<div>
					<p className="eyebrow">Live evidence</p>
					<h2>Processes</h2>
				</div>
				<div className="process-controls">
					<label>
						<span className="sr-only">Search processes</span>
						<input
							type="search"
							aria-label="Search processes"
							placeholder="Search name or PID"
							value={query}
							onChange={(event) => setQuery(event.target.value)}
						/>
					</label>
					<label>
						<span className="sr-only">Sort processes</span>
						<select
							aria-label="Sort processes"
							value={sort}
							onChange={(event) => setSort(event.target.value as SortKey)}
						>
							<option value="cpu">CPU</option>
							<option value="memory">Memory</option>
							<option value="name">Name</option>
							<option value="pid">PID</option>
						</select>
					</label>
				</div>
			</div>

			<div
				className="table-wrap"
				data-testid="process-scroll-region"
				role="region"
				aria-label="Scrollable process list"
				tabIndex={0}
			>
				<table aria-label="Processes" data-testid="process-table">
					<thead>
						<tr>
							<th>Name</th>
							<th>PID</th>
							<th>CPU</th>
							<th>Memory</th>
						</tr>
					</thead>
					<tbody>
						{visible.map((process) => (
							<tr
								key={`${process.key.pid}:${process.key.startedAt}`}
								data-testid="process-row"
								data-process-key={`${process.key.pid}:${process.key.startedAt}`}
								tabIndex={0}
								onClick={() => setSelected(process)}
								onKeyDown={(event) => {
									if (event.key === 'Enter' || event.key === ' ') {
										setSelected(process);
									}
								}}
							>
								<td>{process.name}</td>
								<td>{process.key.pid}</td>
								<td>{formatCpu(process.cpuPercent)}</td>
								<td>{formatBytes(process.memoryBytes)}</td>
							</tr>
						))}
					</tbody>
				</table>
			</div>

			{selected && (
				<div
					className="process-detail-backdrop"
					data-testid="process-detail-backdrop"
					onMouseDown={(event) => {
						if (event.target === event.currentTarget) {
							setSelected(null);
						}
					}}
				>
					<section
						className="process-detail"
						data-testid="process-detail"
						role="dialog"
						aria-modal="true"
						aria-labelledby="process-detail-title"
					>
						<div className="process-detail__header section-heading">
							<div>
								<p className="eyebrow">Process detail</p>
								<h3 id="process-detail-title">{selected.name}</h3>
							</div>
							<button ref={closeButtonRef} type="button" onClick={() => setSelected(null)}>Close</button>
						</div>
						<dl>
							<div><dt>PID</dt><dd>{selected.key.pid}</dd></div>
							<div><dt>Started</dt><dd>{selected.key.startedAt}</dd></div>
							<div><dt>Parent PID</dt><dd>{selected.parentPid ?? 'Unavailable'}</dd></div>
							<div><dt>CPU</dt><dd>{formatCpu(selected.cpuPercent)}</dd></div>
							<div><dt>Memory</dt><dd>{formatBytes(selected.memoryBytes)}</dd></div>
							<div className="detail-path"><dt>Executable</dt><dd>{selected.executablePath ?? 'Unavailable'}</dd></div>
						</dl>
						<div className="history-note" data-testid="process-history">
							<strong>Recent history</strong>
							{history.status === 'loading' && <p>Loading retained samples…</p>}
							{history.status === 'unavailable' && <p>History is unavailable. Live process data remains available.</p>}
							{history.status === 'ready' && history.samples.length === 0 && <p>No retained samples in the last 15 minutes.</p>}
							{history.status === 'ready' && history.samples.length > 0 && (
								<p>{history.samples.length} retained samples · Latest CPU {formatCpu(history.samples.at(-1)?.cpuPercent ?? null)} · Latest memory {formatBytes(history.samples.at(-1)?.memoryBytes ?? null)}</p>
							)}
						</div>
					</section>
				</div>
			)}
		</section>
	);
}
