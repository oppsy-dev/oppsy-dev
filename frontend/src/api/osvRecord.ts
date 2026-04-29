import { Cvss2, Cvss3P1, Cvss4P0 } from 'ae-cvss-calculator';
import type { components } from './schema';

export type OsvSeverityType = components['schemas']['SeverityType'];

export type OsvSeverityEntry = components['schemas']['Severity'];

export type OsvRecordPartial = {
  summary?: string;
  severity?: OsvSeverityEntry[];
};

export type Severity = 'CRITICAL' | 'HIGH' | 'MEDIUM' | 'LOW' | 'NONE';

export function scoreToSeverity(score: number): Severity {
  if (score >= 9.0) return 'CRITICAL';
  if (score >= 7.0) return 'HIGH';
  if (score >= 4.0) return 'MEDIUM';
  if (score > 0.0) return 'LOW';
  return 'NONE';
}

export function ubuntuToSeverity(score: string): Severity {
  switch (score) {
    case 'critical':
      return 'CRITICAL';
    case 'high':
      return 'HIGH';
    case 'medium':
      return 'MEDIUM';
    case 'low':
      return 'LOW';
    default:
      return 'NONE';
  }
}

function severityFromSeverityEntry(entry: OsvSeverityEntry): Severity {
  switch (entry.type) {
    case 'CVSS_V4':
      return scoreToSeverity(new Cvss4P0(entry.score).calculateScores().overall ?? 0);
    case 'CVSS_V3':
      return scoreToSeverity(new Cvss3P1(entry.score).calculateScores().overall ?? 0);
    case 'CVSS_V2':
      return scoreToSeverity(new Cvss2(entry.score).calculateScores().overall ?? 0);
    case 'Ubuntu':
      return ubuntuToSeverity(entry.score);
    default:
      return 'NONE';
  }
}

const SEVERITY_TYPE_PRIORITY: readonly OsvSeverityType[] = [
  'CVSS_V4',
  'CVSS_V3',
  'CVSS_V2',
  'Ubuntu',
];

/**
 * Returns the highest-priority severity entry from the record.
 * Priority order (highest to lowest): CVSS_V4 → CVSS_V3 → CVSS_V2 → Ubuntu.
 */
export function highestPrioritySeverityEntry(
  record: OsvRecordPartial,
): OsvSeverityEntry | undefined {
  if (!record.severity) return undefined;
  for (const type of SEVERITY_TYPE_PRIORITY) {
    const entry = record.severity.find((e) => e.type === type);
    if (entry) return entry;
  }
  return undefined;
}

export function severityFromEntry(record: OsvRecordPartial): Severity {
  const entry = highestPrioritySeverityEntry(record);
  return entry ? severityFromSeverityEntry(entry) : 'NONE';
}
