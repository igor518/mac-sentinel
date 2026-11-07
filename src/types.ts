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

export interface AlertEvidence {
	id: string;
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
	storageStatus: StorageStatus;
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
	subscribe(listener: (snapshot: MonitoringSnapshot) => void): () => void;
	getSettings(): Promise<Settings>;
	saveSettings(settings: Settings): Promise<Settings>;
	deleteHistory(): Promise<void>;
	requestTermination(key: ProcessKey): Promise<TerminationResult>;
}
