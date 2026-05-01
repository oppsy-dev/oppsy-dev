import styles from './WorkspaceSettingsSection.module.css';
import { BackIcon } from '../../../components/Icons';
import { DangerZone } from '../../../components/DangerZone/DangerZone';

type WorkspaceSettingsSectionProps = {
  workspaceName: string;
  onBack: () => void;
  onDelete: () => Promise<void>;
  onDeleted: () => void;
};

export function WorkspaceSettingsSection({
  workspaceName,
  onBack,
  onDelete,
  onDeleted,
}: WorkspaceSettingsSectionProps) {
  return (
    <div>
      <button className={styles.backBtn} type="button" onClick={onBack}>
        <BackIcon width={13} height={13} />
        Back to workspace
      </button>

      <DangerZone
        name={workspaceName}
        title="Delete this workspace"
        description="All manifest files, scan data, and settings will be permanently removed."
        onDelete={onDelete}
        onDeleted={onDeleted}
      />
    </div>
  );
}
