import { invoke } from "@tauri-apps/api/core";
import type {
	CreateDesktopActionResult,
	NewOptionState,
} from "$lib/service-menu/types";
import { buildExec } from "./buildExec";

export const createOption = async (
	newOptionState: NewOptionState,
	launchMode: "command" | "file",
): Promise<CreateDesktopActionResult> => {
	const option = {
		name: newOptionState.name,
		icon: newOptionState.icon,
		exec: buildExec(newOptionState.exec, launchMode),
		type: newOptionState.type,
		mimeType: newOptionState.mimeType.join(";"),
	};

	try {
		const result = await invoke<CreateDesktopActionResult>(
			"create_desktop_action",
			{ action: option },
		);
		return result;
	} catch (error) {
		if (error instanceof Error) {
			console.error(error.message);
		}
		return { success: false, steps: [] };
	}
};
