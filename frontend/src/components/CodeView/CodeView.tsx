import { useState } from 'react';
import styles from './CodeView.module.css';

type CodeViewProps = {
  code: string;
  filename?: string;
};

function CopyIcon() {
  return (
    <svg
      width="13"
      height="13"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth="1.75"
      strokeLinecap="round"
      strokeLinejoin="round"
    >
      <rect x="9" y="9" width="13" height="13" rx="2" />
      <path d="M5 15H4a2 2 0 01-2-2V4a2 2 0 012-2h9a2 2 0 012 2v1" />
    </svg>
  );
}

function CheckIcon() {
  return (
    <svg
      width="13"
      height="13"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth="2.25"
      strokeLinecap="round"
      strokeLinejoin="round"
    >
      <polyline points="20 6 9 17 4 12" />
    </svg>
  );
}

export function CodeView({ code, filename }: CodeViewProps) {
  const [copied, setCopied] = useState(false);
  const lines = code.split('\n');
  const gutterWidth = `${String(lines.length).length}ch`;

  const handleCopy = async () => {
    try {
      await navigator.clipboard.writeText(code);
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    } catch {
      // clipboard unavailable (insecure context or permission denied)
    }
  };

  return (
    <div className={styles.card}>
      <div className={styles.cardHeader}>
        {filename && <span className={styles.label}>{filename}</span>}
        <button
          className={copied ? `${styles.copyBtn} ${styles.copyBtnDone}` : styles.copyBtn}
          type="button"
          onClick={handleCopy}
          aria-label="Copy JSON"
        >
          {copied ? <CheckIcon /> : <CopyIcon />}
          {copied ? 'Copied!' : 'Copy'}
        </button>
      </div>
      <div
        className={styles.codeArea}
        style={{ '--line-num-width': gutterWidth } as React.CSSProperties}
      >
        <div className={styles.lines}>
          {lines.map((line, i) => (
            // eslint-disable-next-line react/no-array-index-key
            <div key={i} className={styles.line}>
              <span className={styles.lineNum}>{i + 1}</span>
              <span className={styles.lineContent}>{line}</span>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}
