import { useState } from 'react';
import { Editor } from '@monaco-editor/react';
import styles from './CodeView.module.css';

export enum CodeType {
  JSON = 'json',
  CUE = 'cue',
}

type CodeViewProps = {
  code: string;
  type: CodeType;
  filename?: string;
  onChange?: (value: string) => void;
  height?: string;
};

function CopyIcon() {
  return (
    <svg
      width="12"
      height="12"
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
      width="12"
      height="12"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth="2.5"
      strokeLinecap="round"
      strokeLinejoin="round"
    >
      <polyline points="20 6 9 17 4 12" />
    </svg>
  );
}

export function CodeView({ code, type, filename, onChange, height = '50vh' }: CodeViewProps) {
  const [copied, setCopied] = useState(false);

  const handleCopy = async () => {
    try {
      await navigator.clipboard.writeText(code);
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    } catch {
      // clipboard unavailable
    }
  };

  const readOnly = onChange === undefined;

  const options = {
    minimap: { enabled: false },
    scrollBeyondLastLine: false,
    fontSize: 13,
    fontFamily: "'JetBrains Mono', monospace",
    padding: { top: 10, bottom: 10 },
    readOnly,
    ...(readOnly ? { readOnlyMessage: { value: '' } } : {}),
  };

  return (
    <div className={styles.card} data-interactive={!readOnly ? true : undefined}>
      {filename && (
        <div className={styles.cardHeader}>
          <span className={styles.fileLabel}>{filename}</span>
          <button
            type="button"
            className={copied ? `${styles.copyBtn} ${styles.copyBtnDone}` : styles.copyBtn}
            onClick={handleCopy}
            aria-label="Copy"
          >
            {copied ? <CheckIcon /> : <CopyIcon />}
            {copied ? 'Copied!' : 'Copy'}
          </button>
        </div>
      )}
      <Editor
        defaultLanguage={type}
        value={code}
        height={height}
        theme="vs"
        options={options}
        onChange={(v) => onChange?.(v ?? '')}
      />
    </div>
  );
}
