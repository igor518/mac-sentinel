import { act, fireEvent, render, screen, within } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';
import App from './App';
import type { MacSentinelBridge, MonitoringSnapshot } from './types';

const snapshot: MonitoringSnapshot = {
	recordedAtMs: 1_794_000_000_000,
	systemCpuPercent: 28,
	usedMemoryBytes: 14_200_000_000,
	processCount: 2,
	samplingGap: false,
	persistenceGap: false,
	storageStatus: 'available',
	notificationsEnabled: null,
	processes: [
		{
			key: { pid: 101, startedAt: 1_793_999_000 },
			name: 'Google Chrome',
			executablePath: '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome',
			parentPid: 1,
			cpuPercent: 141,
			memoryBytes: 2_100_000_000,
		},
		{
			key: { pid: 202, startedAt: 1_793_999_500 },
			name: 'restricted-process',
			executablePath: null,
			parentPid: null,
			cpuPercent: null,
			memoryBytes: 650_000_000,
		},
	],
	alerts: [],
};

function bridge(overrides: Partial<MacSentinelBridge> = {}): MacSentinelBridge {
	return {
		getCurrentSnapshot: vi.fn().mockResolvedValue(snapshot),
		getProcessHistory: vi.fn().mockResolvedValue([]),
		subscribe: vi.fn().mockReturnValue(() => undefined),
		getSettings: vi.fn().mockResolvedValue({}),
		saveSettings: vi.fn().mockResolvedValue({}),
		deleteHistory: vi.fn().mockResolvedValue(undefined),
		requestTermination: vi.fn().mockResolvedValue({ status: 'cancelled' }),
		simulateStorageFailure: vi.fn().mockResolvedValue(undefined),
		simulateSamplingGap: vi.fn().mockResolvedValue(undefined),
		...overrides,
	};
}

describe('MacSentinel application shell', () => {
	it('shows current metrics and explains CPU normalization', async () => {
		render(<App bridge={bridge()} />);

		expect(await screen.findByText('28%')).toBeInTheDocument();
		expect(screen.getByText('14.2 GB')).toBeInTheDocument();
		expect(screen.getByText('2', { selector: '[data-testid="process-count"]' })).toBeInTheDocument();
		expect(screen.getByText(/process CPU may exceed 100%/i)).toBeInTheDocument();
	});

	it('rounds fractional CPU values to the nearest whole percent', async () => {
		render(<App bridge={bridge({
			getCurrentSnapshot: vi.fn().mockResolvedValue({
				...snapshot,
				systemCpuPercent: 19.426672,
			}),
		})} />);

		expect(await screen.findByText('19%')).toBeInTheDocument();
		expect(screen.queryByText('19.426672%')).not.toBeInTheDocument();
	});

	it('searches processes and represents inaccessible values as unavailable', async () => {
		render(<App bridge={bridge()} />);
		await screen.findByText('Google Chrome');

		fireEvent.change(screen.getByRole('searchbox', { name: /processes/i }), {
			target: { value: 'restricted' },
		});

		const table = screen.getByRole('table', { name: /processes/i });
		expect(within(table).queryByText('Google Chrome')).not.toBeInTheDocument();
		expect(within(table).getByText('restricted-process')).toBeInTheDocument();
		expect(within(table).getAllByText('Unavailable').length).toBeGreaterThan(0);
	});

	it('opens process details in a modal and supports every dismissal path', async () => {
		render(<App bridge={bridge()} />);
		await screen.findByText('Google Chrome');

		fireEvent.click(screen.getByText('Google Chrome'));
		let dialog = screen.getByRole('dialog', { name: /google chrome/i });
		expect(within(dialog).getByText('/Applications/Google Chrome.app/Contents/MacOS/Google Chrome')).toBeInTheDocument();

		fireEvent.keyDown(document, { key: 'Escape' });
		expect(screen.queryByRole('dialog')).not.toBeInTheDocument();

		fireEvent.click(screen.getByText('Google Chrome'));
		fireEvent.mouseDown(screen.getByTestId('process-detail-backdrop'));
		expect(screen.queryByRole('dialog')).not.toBeInTheDocument();

		fireEvent.click(screen.getByText('Google Chrome'));
		dialog = screen.getByRole('dialog', { name: /google chrome/i });
		fireEvent.click(within(dialog).getByRole('button', { name: /close/i }));
		expect(screen.queryByRole('dialog')).not.toBeInTheDocument();
	});

	it('loads retained history for the complete process identity', async () => {
		const getProcessHistory = vi.fn().mockResolvedValue([{
			key: snapshot.processes[0].key,
			recordedAtMs: snapshot.recordedAtMs,
			cpuPercent: 88.4,
			memoryBytes: 2_000_000_000,
		}]);
		render(<App bridge={bridge({ getProcessHistory })} />);
		await screen.findByText('Google Chrome');

		fireEvent.click(screen.getByText('Google Chrome'));
		expect(await screen.findByText(/1 retained sample.*Latest CPU 88%/i)).toBeInTheDocument();
		expect(getProcessHistory).toHaveBeenCalledWith(
			snapshot.processes[0].key,
			expect.any(Number),
			expect.any(Number),
		);
	});

	it('keeps live monitoring visible when storage is degraded', async () => {
		render(<App bridge={bridge({
			getCurrentSnapshot: vi.fn().mockResolvedValue({
				...snapshot,
				storageStatus: 'degraded',
			}),
		})} />);

		expect(await screen.findByRole('status', { name: /storage unavailable/i })).toBeInTheDocument();
		expect(screen.getByText('28%')).toBeInTheDocument();
	});

	it('shows the evidence behind a sustained CPU alert', async () => {
		render(<App bridge={bridge({
			getCurrentSnapshot: vi.fn().mockResolvedValue({
				...snapshot,
				alerts: [{
					id: 'cpu:101:1793999000:1794000000000',
					processKey: { pid: 101, startedAt: 1_793_999_000 },
					processName: 'Google Chrome',
					observedCpuPercent: 91,
					thresholdCpuPercent: 80,
					sustainedDurationMs: 600_000,
					episodeStartedAtMs: 1_793_999_400_000,
					triggeredAtMs: 1_794_000_000_000,
					title: 'Sustained CPU usage',
					summary: 'Google Chrome reached 91% CPU against an 80% threshold for a 10 minute duration.',
				}],
			}),
		})} />);

		const alertCenter = await screen.findByTestId('alert-center');
		expect(within(alertCenter).getByTestId('alert-evidence')).toHaveTextContent('Google Chrome');
		expect(within(alertCenter).getByTestId('alert-evidence')).toHaveTextContent('80% threshold');
		expect(within(alertCenter).getByTestId('alert-evidence')).toHaveTextContent('10 minute duration');
	});

	it('keeps process scrolling separate and renders bounded metric trends', async () => {
		let publish: ((next: MonitoringSnapshot) => void) | undefined;
		const testBridge = bridge({
			subscribe: vi.fn((listener) => {
				publish = listener;
				return () => undefined;
			}),
		});
		render(<App bridge={testBridge} />);

		expect(await screen.findByTestId('process-scroll-region')).toBeInTheDocument();
		expect(screen.getByRole('img', { name: /CPU trend/i })).toHaveAttribute('data-point-count', '1');
		expect(screen.getByRole('img', { name: /Memory trend/i })).toHaveAttribute('data-point-count', '1');

		act(() => publish?.({
			...snapshot,
			recordedAtMs: snapshot.recordedAtMs + 2_000,
			systemCpuPercent: 35,
			usedMemoryBytes: 15_000_000_000,
		}));

		expect(screen.getByRole('img', { name: /CPU trend/i })).toHaveAttribute('data-point-count', '2');
		expect(screen.getByRole('img', { name: /Memory trend/i })).toHaveAttribute('data-point-count', '2');

		for (let index = 2; index < 65; index += 1) {
			act(() => publish?.({
				...snapshot,
				recordedAtMs: snapshot.recordedAtMs + index * 2_000,
				systemCpuPercent: 20 + index,
				usedMemoryBytes: 14_000_000_000 + index * 10_000_000,
			}));
		}
		expect(screen.getByRole('img', { name: /CPU trend/i })).toHaveAttribute('data-point-count', '60');
		expect(screen.getByRole('img', { name: /Memory trend/i })).toHaveAttribute('data-point-count', '60');

		act(() => publish?.({
			...snapshot,
			recordedAtMs: snapshot.recordedAtMs + 65 * 2_000,
			systemCpuPercent: null,
			usedMemoryBytes: null,
		}));
		expect(screen.getByRole('img', { name: /CPU trend/i })).toHaveAttribute('data-point-count', '59');
		expect(screen.getByRole('img', { name: /Memory trend/i })).toHaveAttribute('data-point-count', '59');
	});
});
