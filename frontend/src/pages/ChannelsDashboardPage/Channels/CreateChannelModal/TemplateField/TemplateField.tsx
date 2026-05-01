import { useRef, useState } from 'react';
import { CodeType, CodeView } from '../../../../../components/CodeView/CodeView';
import styles from './TemplateField.module.css';

const SCHEMA = `_workspace_id: string
_workspace_name: string
_manifest_id: string
_manifest_type: string
_manifest_name: string
_manifest_tag?: string | null
_osv_records: [string, ...string]`;

function InfoIcon() {
  return (
    <svg
      width="11"
      height="11"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth="2"
      strokeLinecap="round"
      strokeLinejoin="round"
    >
      <circle cx="12" cy="12" r="10" />
      <line x1="12" y1="16" x2="12" y2="12" />
      <line x1="12" y1="8" x2="12.01" y2="8" />
    </svg>
  );
}

function ChevronIcon({ open }: { open: boolean }) {
  return (
    <svg
      width="10"
      height="10"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth="2.5"
      strokeLinecap="round"
      strokeLinejoin="round"
      style={{ transform: open ? 'rotate(180deg)' : 'none', transition: 'transform 0.18s ease' }}
    >
      <polyline points="6 9 12 15 18 9" />
    </svg>
  );
}

type TemplateFieldProps = {
  value: string;
  onChange: (value: string) => void;
  templateSchema?: string;
  alwaysOpen?: boolean;
};

export function TemplateField({ value, onChange, templateSchema, alwaysOpen }: TemplateFieldProps) {
  const [open, setOpen] = useState(alwaysOpen ?? false);
  const [schemaVisible, setSchemaVisible] = useState(false);
  const [templateSchemaVisible, setTemplateSchemaVisible] = useState(false);
  const hideTimer = useRef<number>(undefined);
  const hideTemplateTimer = useRef<number>(undefined);

  const showSchema = () => {
    window.clearTimeout(hideTimer.current);
    setSchemaVisible(true);
  };

  const hideSchema = () => {
    hideTimer.current = window.setTimeout(() => setSchemaVisible(false), 150);
  };

  const showTemplateSchema = () => {
    window.clearTimeout(hideTemplateTimer.current);
    setTemplateSchemaVisible(true);
  };

  const hideTemplateSchema = () => {
    hideTemplateTimer.current = window.setTimeout(() => setTemplateSchemaVisible(false), 150);
  };

  return (
    <div className={styles.field}>
      <div className={styles.fieldToggleRow}>
        <span className={styles.fieldLabel}>Payload template</span>
        {!alwaysOpen && (
          <button
            type="button"
            className={styles.fieldToggleBtn}
            onClick={() => setOpen((o) => !o)}
          >
            Customize <ChevronIcon open={open} />
          </button>
        )}
      </div>

      {open && (
        <>
          <a
            href="https://cuelang.org/docs/reference/spec/#introduction"
            target="_blank"
            rel="noreferrer"
            className={styles.schemaTooltipLink}
          >
            Templating is resolved with the help of CUE ↗
          </a>
          <div className={styles.schemaInfoRow}>
            <div
              className={styles.schemaInfoAnchor}
              onMouseEnter={showSchema}
              onMouseLeave={hideSchema}
            >
              <span>Available fields</span>
              <InfoIcon />

              {schemaVisible && (
                <div
                  className={styles.schemaTooltip}
                  onMouseEnter={showSchema}
                  onMouseLeave={hideSchema}
                >
                  <CodeView code={SCHEMA} type={CodeType.CUE} height="10rem" />
                </div>
              )}
            </div>

            {templateSchema && (
              <div
                className={styles.schemaInfoAnchor}
                onMouseEnter={showTemplateSchema}
                onMouseLeave={hideTemplateSchema}
              >
                <span>Template schema</span>
                <InfoIcon />

                {templateSchemaVisible && (
                  <div
                    className={styles.schemaTooltip}
                    onMouseEnter={showTemplateSchema}
                    onMouseLeave={hideTemplateSchema}
                  >
                    <CodeView code={templateSchema} type={CodeType.CUE} height="5rem" />
                  </div>
                )}
              </div>
            )}
          </div>

          <CodeView code={value} type={CodeType.CUE} height="14rem" onChange={onChange} />
        </>
      )}
    </div>
  );
}
