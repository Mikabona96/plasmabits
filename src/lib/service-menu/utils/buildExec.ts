const shellQuote = (value: string): string => {
	return `'${value.replace(/'/g, `'\\''`)}'`;
};

export const buildExec = (command: string, mode: "command" | "file") => {
	if (mode === "file") return `${command} %f`;
	return `sh -c ${shellQuote(command)}`;
};
