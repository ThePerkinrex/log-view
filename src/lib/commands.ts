import { invoke } from "@tauri-apps/api/core";
import { Record } from "./record";

export async function get_recent_files(): Promise<string[]> {
	return await invoke("get_recent_files", {})
}

export async function get_open_files(): Promise<string[]> {
	return await invoke("get_open_files", {})
}

export async function select_file(): Promise<string | null | undefined> {
	return await invoke("select_file", {})
}

export async function open_file(path: string): Promise<string | null | undefined> {
	return await invoke("open_file", {path})
}


export async function close_file(file: string): Promise<void> {
	return await invoke("close_file", {file})
}

export async function remove_recent_file(file: string): Promise<void> {
	return await invoke("remove_recent_file", {file})
}

export async function get_file(path: string): Promise<Record[] | null | undefined> {
	return await invoke("get_file", {path})
}
