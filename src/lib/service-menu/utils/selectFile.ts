import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import type { AppMetadata, NewOptionState } from "$lib/service-menu/types";

export const selectFile = async (newOptionState: NewOptionState) => {
	const selected = await open({
		multiple: false,
		directory: false,
		defaultPath: "/usr/share/applications",
	});

	if (typeof selected !== "string") return;

	const metadata = await invoke<AppMetadata | null>("get_app_metadata", {
		desktopFile: selected,
	});

	if (!metadata) return;

	if (metadata.name) newOptionState.name = `Open with ${metadata.name}`;
	if (metadata.icon) newOptionState.icon = metadata.icon;
	if (metadata.exec) newOptionState.exec = metadata.exec;
};
