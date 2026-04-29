import { useRef, useState } from 'react';
import { useUploadManifest } from '../../../../hooks/workspaces';
import type { ManifestType } from '../../../../api/workspaces';
import styles from './UploadManifestModal.module.css';
import { XIcon } from '../../../../components/Icons';

const MANIFEST_TYPES: { value: ManifestType; label: string }[] = [
  { value: 'Cargo', label: 'Cargo.lock (Rust)' },
  { value: 'Npm', label: 'package-lock.json (Node.js)' },
  { value: 'Poetry', label: 'poetry.lock (Python)' },
  { value: 'Uv', label: 'uv.lock (Python / uv)' },
  { value: 'Go', label: 'go.sum (Go)' },
];

const FILENAME_TO_TYPE: Record<string, ManifestType> = {
  'cargo.lock': 'Cargo',
  'package-lock.json': 'Npm',
  'poetry.lock': 'Poetry',
  'uv.lock': 'Uv',
  'go.sum': 'Go',
};

type UploadManifestModalProps = {
  workspaceId: string;
  onClose: () => void;
};

function UploadCloudIcon() {
  return (
    <svg
      width="28"
      height="28"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth="1.5"
      strokeLinecap="round"
      strokeLinejoin="round"
    >
      <polyline points="16 16 12 12 8 16" />
      <line x1="12" y1="12" x2="12" y2="21" />
      <path d="M20.39 18.39A5 5 0 0018 9h-1.26A8 8 0 103 16.3" />
    </svg>
  );
}

function FileIcon() {
  return (
    <svg
      width="22"
      height="22"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth="1.5"
      strokeLinecap="round"
      strokeLinejoin="round"
    >
      <path d="M14 2H6a2 2 0 00-2 2v16a2 2 0 002 2h12a2 2 0 002-2V8z" />
      <polyline points="14 2 14 8 20 8" />
    </svg>
  );
}

export function UploadManifestModal({ workspaceId, onClose }: UploadManifestModalProps) {
  const [file, setFile] = useState<File | null>(null);
  const [manifestType, setManifestType] = useState<ManifestType | ''>('');
  const [name, setName] = useState('');
  const [tag, setTag] = useState('');
  const [dragOver, setDragOver] = useState(false);
  const inputRef = useRef<HTMLInputElement>(null);

  const uploadMutation = useUploadManifest(workspaceId);

  const handleFile = (f: File) => {
    setFile(f);
    setName(f.name);
    const detected = FILENAME_TO_TYPE[f.name.toLowerCase()];
    if (detected) setManifestType(detected);
  };

  const handleDrop = (e: React.DragEvent) => {
    e.preventDefault();
    setDragOver(false);
    const f = e.dataTransfer.files[0];
    if (f) handleFile(f);
  };

  const handleUpload = async () => {
    if (!file || !manifestType || !name.trim()) return;
    await uploadMutation.mutateAsync({
      file,
      manifest_type: manifestType,
      name: name.trim(),
      tag: tag.trim() || undefined,
    });
    onClose();
  };

  const canUpload =
    file !== null && manifestType !== '' && name.trim() !== '' && !uploadMutation.isPending;

  return (
    <div className={styles.backdrop}>
      <div
        className={styles.modal}
        role="dialog"
        aria-modal="true"
        aria-labelledby="upload-modal-title"
      >
        <div className={styles.modalHeader}>
          <h2 id="upload-modal-title" className={styles.modalTitle}>
            Upload Manifest
          </h2>
          <button className={styles.closeBtn} type="button" onClick={onClose} aria-label="Close">
            <XIcon width={16} height={16} />
          </button>
        </div>

        <div className={styles.body}>
          <div
            className={[
              styles.dropZone,
              dragOver ? styles.dropZoneOver : '',
              file ? styles.dropZoneHasFile : '',
            ]
              .filter(Boolean)
              .join(' ')}
            onDragOver={(e) => {
              e.preventDefault();
              setDragOver(true);
            }}
            onDragLeave={() => setDragOver(false)}
            onDrop={handleDrop}
            onClick={() => inputRef.current?.click()}
            role="button"
            tabIndex={0}
          >
            <input
              ref={inputRef}
              type="file"
              className={styles.fileInput}
              onChange={(e) => {
                const f = e.target.files?.[0];
                if (f) handleFile(f);
              }}
            />
            <div className={styles.dropIcon}>{file ? <FileIcon /> : <UploadCloudIcon />}</div>
            {file ? (
              <p className={styles.fileName}>{file.name}</p>
            ) : (
              <>
                <p className={styles.dropTitle}>Drop your lock file here</p>
                <p className={styles.dropSub}>or click to browse</p>
              </>
            )}
          </div>

          <div className={styles.field}>
            <label className={styles.fieldLabel} htmlFor="manifest-type">
              Ecosystem
            </label>
            <select
              id="manifest-type"
              className={styles.select}
              value={manifestType}
              onChange={(e) => setManifestType(e.target.value as ManifestType)}
            >
              <option value="" disabled>
                Select ecosystem…
              </option>
              {MANIFEST_TYPES.map(({ value, label }) => (
                <option key={value} value={value}>
                  {label}
                </option>
              ))}
            </select>
          </div>

          <div className={styles.field}>
            <label className={styles.fieldLabel} htmlFor="manifest-name">
              Name
            </label>
            <input
              id="manifest-name"
              type="text"
              className={styles.input}
              value={name}
              onChange={(e) => setName(e.target.value)}
              placeholder="e.g. Cargo.lock"
            />
          </div>

          <div className={styles.field}>
            <label className={styles.fieldLabel} htmlFor="manifest-tag">
              Tag <span className={styles.fieldLabelOptional}>(optional)</span>
            </label>
            <input
              id="manifest-tag"
              type="text"
              className={styles.input}
              value={tag}
              onChange={(e) => setTag(e.target.value)}
              placeholder="e.g. production"
            />
          </div>

          {uploadMutation.isError && (
            <p className={styles.error}>Upload failed. Check that the backend is running.</p>
          )}
        </div>

        <div className={styles.modalFooter}>
          <button
            className={styles.cancelBtn}
            type="button"
            onClick={onClose}
            disabled={uploadMutation.isPending}
          >
            Cancel
          </button>
          <button
            className={styles.confirmBtn}
            type="button"
            onClick={handleUpload}
            disabled={!canUpload}
          >
            {uploadMutation.isPending ? 'Uploading…' : 'Upload'}
          </button>
        </div>
      </div>
    </div>
  );
}
