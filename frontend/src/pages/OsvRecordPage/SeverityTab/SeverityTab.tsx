import { useRef, useState } from 'react';
import type { CSSProperties, ReactNode } from 'react';
import type { OsvRecord } from '../cveUtils';
import { Cvss4Card } from './Cvss4Card/Cvss4Card';
import { Cvss3Card } from './Cvss3Card/Cvss3Card';
import { Cvss2Card } from './Cvss2Card/Cvss2Card';
import { UbuntuCard } from './UbuntuCard/UbuntuCard';
import styles from './SeverityTab.module.css';

type SeverityType = 'CVSS_V4' | 'CVSS_V3' | 'CVSS_V2' | 'Ubuntu';

type SeveritySelection = {
  type: SeverityType;
  card: ReactNode;
  pillStyle: CSSProperties;
};

const SEVERITY_LABELS: Record<SeverityType, string> = {
  CVSS_V4: 'CVSS v4',
  CVSS_V3: 'CVSS v3',
  CVSS_V2: 'CVSS v2',
  Ubuntu: 'Ubuntu',
};

const SEVERITY_PRIORITY: SeverityType[] = ['CVSS_V4', 'CVSS_V3', 'CVSS_V2', 'Ubuntu'];

function buildCard(record: OsvRecord, type: SeverityType): ReactNode {
  const entry = record.severity?.find((s) => s.type === type);
  if (!entry) return null;
  switch (entry.type) {
    case 'CVSS_V4':
      return <Cvss4Card score={entry.score} />;
    case 'CVSS_V3':
      return <Cvss3Card score={entry.score} />;
    case 'CVSS_V2':
      return <Cvss2Card score={entry.score} />;
    case 'Ubuntu':
      return <UbuntuCard score={entry.score} />;
    default:
      return null;
  }
}

type SeverityTabProps = {
  record: OsvRecord;
};

export function SeverityTab({ record }: SeverityTabProps) {
  const availableTypes = SEVERITY_PRIORITY.filter((type) =>
    record.severity?.some((s) => s.type === type),
  );

  const [selection, setSelection] = useState<SeveritySelection>(() => ({
    type: availableTypes[0] ?? 'CVSS_V4',
    card: availableTypes.length > 0 ? buildCard(record, availableTypes[0]!) : null,
    pillStyle: {},
  }));

  const btnRefs = useRef<(HTMLButtonElement | null)[]>([]);

  if (availableTypes.length === 0) {
    return <p className={styles.empty}>No severity data available.</p>;
  }

  function handleTypeSelect(type: SeverityType) {
    const idx = availableTypes.indexOf(type);
    const btn = btnRefs.current[idx];
    const pillStyle = btn ? { left: btn.offsetLeft, width: btn.offsetWidth } : selection.pillStyle;
    setSelection({ type, card: buildCard(record, type), pillStyle });
  }

  return (
    <div>
      {availableTypes.length > 1 && (
        <div className={styles.toggle}>
          <div className={styles.togglePill} style={selection.pillStyle} />
          {availableTypes.map((type, idx) => (
            <button
              key={type}
              ref={(el) => {
                btnRefs.current[idx] = el;
              }}
              type="button"
              className={`${styles.toggleBtn}${selection.type === type ? ` ${styles.toggleBtnActive}` : ''}`}
              onClick={() => handleTypeSelect(type)}
            >
              {SEVERITY_LABELS[type]}
            </button>
          ))}
        </div>
      )}
      {selection.card}
    </div>
  );
}
