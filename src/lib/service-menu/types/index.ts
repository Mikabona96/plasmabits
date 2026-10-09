export interface AppMetadata {
	name: string;
	icon: string;
	exec: string;
}

export interface NewOptionState {
	type: string;
	mimeType: string[];
	name: string;
	icon: string;
	exec: string;
}

interface OperationStep {
	name: string;
	success: boolean | null;
	message: string;
}

export interface CreateDesktopActionResult {
	success: boolean;
	steps: OperationStep[];
}

interface OperationStep {
	name: string;
	success: boolean | null;
	message: string;
}

export interface CreateDesktopActionResult {
	success: boolean;
	steps: OperationStep[];
}
