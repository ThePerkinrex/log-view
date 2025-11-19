// Matches RecordExtra
export interface RecordExtra {
	task_id?: string | null;
	process_id?: string | null;
}

// Matches serde_json::Value
export type JsonValue =
	| null
	| string
	| number
	| boolean
	| JsonValue[]
	| { [key: string]: JsonValue };

// Matches Record
export interface Record {
	level: string;
	target: string;
	message: string;
	module_path?: string | null;
	file?: string | null;
	line?: number | null;
	data: { [key: string]: JsonValue };
	extra: RecordExtra; // serde(default) → always present
}
