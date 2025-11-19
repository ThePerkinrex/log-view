const palette = [
  "#9cdcfe", "#ce9178", "#c586c0", "#dcdcaa", "#4ec9b0",
  "#569cd6", "#b5cea8", "#d16969", "#ffd700", "#ff8c00"
];

function hash(s: string): number {
  let h = 0;
  for (let i = 0; i < s.length; i++) h = (h * 31 + s.charCodeAt(i)) | 0;
  return Math.abs(h);
}

export function colorFor(id: string | null | undefined): string {
  if (!id) return "#888"; // default color
  return palette[hash(id) % palette.length];
}
