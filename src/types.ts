export interface ProcessKey {
	pid: number;
	startedAt: number;
}

export interface ProcessSnapshot {
	key: ProcessKey;
	name: string;
	executablePath: string | null;
	parentPid: number | null;
	cpuPercent: number | null;
	memoryBytes: number;
}

export interface ProcessSample {
	key: ProcessKey;
	recordedAtMs: number;
	cpuPercent: number | null;
	memoryBytes: number;
}

export interface AlertEvidence {
	id: string;
	processKey: ProcessKey;
	processName: string;
	observedCpuPercent: number;
	thresholdCpuPercent: number;
	sustainedDurationMs: number;
	episodeStartedAtMs: number;
	triggeredAtMs: number;
	title: string;
	summary: string;
}

export type StorageStatus = 'available' | 'degraded' | 'unavailable';

export interface MonitoringSnapshot {
	recordedAtMs: number;
	systemCpuPercent: number | null;
	usedMemoryBytes: number | null;
	processCount: number | null;
	samplingGap: boolean;
	persistenceGap: boolean;
	storageStatus: StorageStatus;
	notificationsEnabled: boolean | null;
	processes: ProcessSnapshot[];
	alerts: AlertEvidence[];
}

export interface Settings {
	[key: string]: unknown;
}

export interface TerminationResult {
	status: string;
}

export interface MacSentinelBridge {
	getCurrentSnapshot(): Promise<MonitoringSnapshot>;
	getProcessHistory(key: ProcessKey, fromMs: number, toMs: number): Promise<ProcessSample[]>;
	subscribe(listener: (snapshot: MonitoringSnapshot) => void): () => void;
	getSettings(): Promise<Settings>;
	saveSettings(settings: Settings): Promise<Settings>;
	deleteHistory(): Promise<void>;
	requestTermination(key: ProcessKey): Promise<TerminationResult>;
	simulateStorageFailure(): Promise<void>;
	simulateSamplingGap(): Promise<void>;
}
