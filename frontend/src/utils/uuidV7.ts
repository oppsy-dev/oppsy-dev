export function formatUuidV7TimeAgo(uuidV7: string): string {
  const ms = parseInt(uuidV7.replace(/-/g, '').slice(0, 12), 16);
  const diffMs = Date.now() - ms;
  const diffSecs = Math.floor(diffMs / 1000);
  const diffMins = Math.floor(diffSecs / 60);
  const diffHours = Math.floor(diffMins / 60);
  const diffDays = Math.floor(diffHours / 24);

  if (diffSecs < 60) return 'just now';
  if (diffMins < 60) return `${diffMins}m ago`;
  if (diffHours < 24) return `${diffHours}h ago`;
  if (diffDays < 30) return `${diffDays}d ago`;
  return formatUuidV7Date(uuidV7);
}

export function formatUuidV7Date(uuidV7: string, includeTime?: true): string {
  const ms = parseInt(uuidV7.replace(/-/g, '').slice(0, 12), 16);
  const date = new Date(ms);

  if (includeTime) {
    return date.toLocaleString(undefined, {
      month: 'short',
      day: 'numeric',
      year: 'numeric',
      hour: 'numeric',
      minute: '2-digit',
    });
  }

  return date.toLocaleDateString('en-US', {
    month: 'short',
    day: 'numeric',
    year: 'numeric',
  });
}
