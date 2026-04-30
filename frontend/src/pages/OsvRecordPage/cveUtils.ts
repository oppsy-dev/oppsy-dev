import type { components, paths } from '../../api/schema';

export type OsvRecord =
  paths['/v1/osv/{record_id}']['get']['responses']['200']['content']['application/json; charset=utf-8'];

export type OsvAffected = components['schemas']['Affected'];
export type OsvPackage = components['schemas']['Package'];
export type OsvRange = components['schemas']['Range'];
export type OsvEvent = components['schemas']['Event'];
export type OsvReference = components['schemas']['Reference'];
export type OsvCreditType = components['schemas']['CreditType'];
export type OsvCredit = components['schemas']['Credit'];
export type Severity = components['schemas']['Severity'];

export function formatDate(isoString?: string): string {
  if (!isoString) return '';
  return isoString.slice(0, 10);
}

export function formatRangeLabel(range: OsvRange): string {
  const introduced = range.events.find((e) => e.introduced)?.introduced;
  const fixed = range.events.find((e) => e.fixed)?.fixed;
  const lastAffected = range.events.find((e) => e.last_affected)?.last_affected;

  if (introduced && fixed) return `${introduced} → ${fixed}`;
  if (introduced && lastAffected) return `${introduced} → ${lastAffected} (last)`;
  if (introduced) return `≥ ${introduced}`;
  return '—';
}

export function formatCreditType(type?: OsvCreditType): string {
  if (!type) return '—';
  return type
    .split('_')
    .map((w) => w.charAt(0) + w.slice(1).toLowerCase())
    .join(' ');
}
