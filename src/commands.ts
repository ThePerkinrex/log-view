import { invoke } from "@tauri-apps/api/core";
import { Record } from "./record";

export async function get_recent_files(): Promise<string[]> {
	return await invoke("get_recent_files", {})
}

export async function get_open_files(): Promise<string[]> {
	return await invoke("get_open_files", {})
}

export async function open_file(): Promise<string | null | undefined> {
	return await invoke("open_file", {})
}

export async function get_file(path: string): Promise<Record[] | null | undefined> {
	return await invoke("get_file", {path})
}
