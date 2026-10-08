export const mimeTypeCategories = [
	{
		name: "Folders",
		types: [{ name: "Folder", mimeType: "inode/directory" }],
	},

	{
		name: "Images",
		types: [
			{ name: "JPEG image", mimeType: "image/jpeg" },
			{ name: "PNG image", mimeType: "image/png" },
			{ name: "GIF image", mimeType: "image/gif" },
			{ name: "SVG image", mimeType: "image/svg+xml" },
			{ name: "WebP image", mimeType: "image/webp" },
			{ name: "BMP image", mimeType: "image/bmp" },
			{ name: "TIFF image", mimeType: "image/tiff" },
			{ name: "AVIF image", mimeType: "image/avif" },
			{ name: "HEIC image", mimeType: "image/heic" },
			{ name: "HEIF image", mimeType: "image/heif" },
			{ name: "ICO image", mimeType: "image/vnd.microsoft.icon" },
			{ name: "APNG image", mimeType: "image/apng" },
		],
	},

	{
		name: "Videos",
		types: [
			{ name: "MP4 video", mimeType: "video/mp4" },
			{ name: "WebM video", mimeType: "video/webm" },
			{ name: "Matroska video", mimeType: "video/matroska" },
			{ name: "Ogg video", mimeType: "video/ogg" },
			{ name: "AVI video", mimeType: "video/vnd.avi" },
			{ name: "QuickTime video", mimeType: "video/quicktime" },
			{ name: "3GP video", mimeType: "video/3gpp" },
			{ name: "Flash video", mimeType: "video/x-flv" },
		],
	},

	{
		name: "Audio",
		types: [
			{ name: "MP3 audio", mimeType: "audio/mpeg" },
			{ name: "WAV audio", mimeType: "audio/vnd.wave" },
			{ name: "Ogg audio", mimeType: "audio/ogg" },
			{ name: "FLAC audio", mimeType: "audio/flac" },
			{ name: "AAC audio", mimeType: "audio/aac" },
			{ name: "Opus audio", mimeType: "audio/ogg" },
			{ name: "MIDI audio", mimeType: "audio/midi" },
			{ name: "AIFF audio", mimeType: "audio/x-aiff" },
			{ name: "WebM audio", mimeType: "audio/webm" },
			{ name: "M4A audio", mimeType: "audio/mp4" },
		],
	},

	{
		name: "Documents",
		types: [
			{ name: "PDF document", mimeType: "application/pdf" },
			{ name: "Microsoft Word document", mimeType: "application/msword" },
			{
				name: "Microsoft Word document (DOCX)",
				mimeType:
					"application/vnd.openxmlformats-officedocument.wordprocessingml.document",
			},
			{
				name: "Microsoft Excel spreadsheet",
				mimeType: "application/vnd.ms-excel",
			},
			{
				name: "Microsoft Excel spreadsheet (XLSX)",
				mimeType:
					"application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
			},
			{
				name: "Microsoft PowerPoint presentation",
				mimeType: "application/vnd.ms-powerpoint",
			},
			{
				name: "Microsoft PowerPoint presentation (PPTX)",
				mimeType:
					"application/vnd.openxmlformats-officedocument.presentationml.presentation",
			},
			{
				name: "OpenDocument Text",
				mimeType: "application/vnd.oasis.opendocument.text",
			},
			{
				name: "OpenDocument Spreadsheet",
				mimeType: "application/vnd.oasis.opendocument.spreadsheet",
			},
			{
				name: "OpenDocument Presentation",
				mimeType: "application/vnd.oasis.opendocument.presentation",
			},
			{ name: "RTF document", mimeType: "application/rtf" },
			{ name: "EPUB ebook", mimeType: "application/epub+zip" },
		],
	},

	{
		name: "Text",
		types: [
			{ name: "Plain text", mimeType: "text/plain" },
			{ name: "CSV", mimeType: "text/csv" },
			{ name: "HTML", mimeType: "text/html" },
			{ name: "CSS", mimeType: "text/css" },
			{ name: "XML", mimeType: "application/xml" },
			{ name: "JSON", mimeType: "application/json" },
			{ name: "YAML", mimeType: "text/yaml" },
			{ name: "JavaScript", mimeType: "text/javascript" },
			{ name: "TypeScript", mimeType: "text/plain" },
			{ name: "Markdown", mimeType: "text/markdown" },
		],
	},

	{
		name: "Code",
		types: [
			{ name: "C source", mimeType: "text/x-csrc" },
			{ name: "C++ source", mimeType: "text/x-c++src" },
			{ name: "Java source", mimeType: "text/x-java" },
			{ name: "Python source", mimeType: "text/x-python" },
			{ name: "PHP source", mimeType: "text/x-php" },
			{ name: "Ruby source", mimeType: "text/x-ruby" },
			{ name: "Shell script", mimeType: "text/x-shellscript" },
			{ name: "Rust source", mimeType: "text/rust" },
			{ name: "Go source", mimeType: "text/x-go" },
			{ name: "Makefile", mimeType: "text/x-makefile" },
			{ name: "CMake file", mimeType: "text/x-cmake" },
			{ name: "SQL", mimeType: "application/sql" },
		],
	},

	{
		name: "Archives",
		types: [
			{ name: "ZIP archive", mimeType: "application/zip" },
			{ name: "GZIP archive", mimeType: "application/gzip" },
			{ name: "BZIP2 archive", mimeType: "application/x-bzip2" },
			{ name: "XZ archive", mimeType: "application/x-xz" },
			{ name: "7-Zip archive", mimeType: "application/x-7z-compressed" },
			{ name: "RAR archive", mimeType: "application/vnd.rar" },
			{ name: "TAR archive", mimeType: "application/x-tar" },
			{
				name: "Zstandard compressed file",
				mimeType: "application/zstd",
			},
			{
				name: "Debian package",
				mimeType: "application/vnd.debian.binary-package",
			},
			{ name: "RPM package", mimeType: "application/x-rpm" },
		],
	},

	{
		name: "Fonts",
		types: [
			{ name: "TrueType font", mimeType: "font/ttf" },
			{ name: "OpenType font", mimeType: "font/otf" },
			{ name: "WOFF font", mimeType: "font/woff" },
			{ name: "WOFF2 font", mimeType: "font/woff2" },
		],
	},

	{
		name: "Disk images",
		types: [
			{
				name: "ISO disk image",
				mimeType: "application/x-cd-image",
			},
		],
	},
];
