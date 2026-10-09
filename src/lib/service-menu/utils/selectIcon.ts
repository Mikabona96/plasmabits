import { open } from "@tauri-apps/plugin-dialog";
import type { NewOptionState } from "$lib/service-menu/types";

export const selectIcon = async (newOptionState: NewOptionState) => {
	const selected = await open({
		multiple: false,
		directory: false,
		filters: [{ name: "Images", extensions: ["png", "svg"] }],
	});

	if (typeof selected !== "string") return;
	newOptionState.icon = selected;
};
