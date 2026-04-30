import { useState } from 'react';
import type { CreateChannelRequest } from '../../../../api/notification_channels';
import {
  ChannelTypePicker,
  CHANNEL_TYPES,
  type NotificationChannelType,
} from './ChannelTypePicker/ChannelTypePicker';
import {
  DiscordChannelForm,
  buildDiscordChannel,
  type DiscordFormState,
} from './DiscordChannelForm/DiscordChannelForm';
import {
  WebhookChannelForm,
  buildWebhookChannel,
  type WebhookFormState,
} from './WebhookChannelForm/WebhookChannelForm';
import {
  EmailChannelForm,
  buildEmailChannel,
  type EmailFormState,
} from './EmailChannelForm/EmailChannelForm';
import styles from './CreateChannelModal.module.css';
import { BackIcon, XIcon } from '../../../../components/Icons';

type FormStates = {
  Discord: DiscordFormState;
  Webhook: WebhookFormState;
  Email: EmailFormState;
};

const INITIAL_FORM_STATES: FormStates = {
  Discord: { name: '', webhookUrl: '' },
  Webhook: { name: '', url: '', secret: '' },
  Email: { name: '', from: '', addresses: [''] },
};

function buildChannel(
  selectedType: NotificationChannelType,
  forms: FormStates,
): CreateChannelRequest | null {
  switch (selectedType) {
    case 'Discord':
      return buildDiscordChannel(forms.Discord);
    case 'Webhook':
      return buildWebhookChannel(forms.Webhook);
    case 'Email':
      return buildEmailChannel(forms.Email);
  }
}

type AddChannelModalProps = {
  onClose: () => void;
  onSuccess: (body: CreateChannelRequest) => void;
};

export function CreateChannelModal({ onClose, onSuccess }: AddChannelModalProps) {
  const [selectedType, setSelectedType] = useState<NotificationChannelType | null>(null);
  const [creating, setCreating] = useState(false);
  const [hasError, setHasError] = useState(false);

  const [forms, setForms] = useState<FormStates>(INITIAL_FORM_STATES);
  const setForm =
    <K extends keyof FormStates>(key: K) =>
    (value: FormStates[K]) =>
      setForms((prev) => ({ ...prev, [key]: value }));

  const handleBack = () => {
    setHasError(false);
    setSelectedType(null);
  };

  const handleAdd = async () => {
    if (!selectedType) return;
    const reqBody = buildChannel(selectedType, forms);
    if (!reqBody) return;

    setCreating(true);
    setHasError(false);
    try {
      onSuccess(reqBody);
    } catch {
      setHasError(true);
      setCreating(false);
    }
  };

  const canAdd = !creating && selectedType !== null && buildChannel(selectedType, forms) !== null;

  const selectedLabel = CHANNEL_TYPES.find((t) => t.id === selectedType)?.label;

  return (
    <div className={styles.backdrop}>
      <div className={styles.modal} role="dialog" aria-modal="true">
        <div className={styles.modalHeader}>
          <div className={styles.modalHeaderLeft}>
            {selectedType !== null && (
              <button
                type="button"
                className={styles.backBtn}
                onClick={handleBack}
                aria-label="Back to channel type selection"
              >
                <BackIcon width={13} height={13} />
              </button>
            )}
            <h2 className={styles.modalTitle}>
              {selectedLabel ? `Add ${selectedLabel} Channel` : 'Add Notification Channel'}
            </h2>
          </div>
          <button className={styles.closeBtn} type="button" onClick={onClose} aria-label="Close">
            <XIcon width={16} height={16} />
          </button>
        </div>

        <div className={styles.body}>
          {selectedType === null ? (
            <ChannelTypePicker onSelect={setSelectedType} />
          ) : (
            <div className={styles.formBody}>
              {selectedType === 'Discord' && (
                <DiscordChannelForm value={forms.Discord} onChange={setForm('Discord')} />
              )}
              {selectedType === 'Webhook' && (
                <WebhookChannelForm value={forms.Webhook} onChange={setForm('Webhook')} />
              )}
              {selectedType === 'Email' && (
                <EmailChannelForm value={forms.Email} onChange={setForm('Email')} />
              )}
              {hasError && (
                <p style={{ fontSize: '0.8125rem', color: 'var(--error, #dc2626)', margin: 0 }}>
                  Failed to add channel. Please try again.
                </p>
              )}
            </div>
          )}
        </div>

        {selectedType !== null && (
          <div className={styles.modalFooter}>
            <button
              className={styles.cancelBtn}
              type="button"
              onClick={onClose}
              disabled={creating}
            >
              Cancel
            </button>
            <button
              className={styles.confirmBtn}
              type="button"
              onClick={handleAdd}
              disabled={!canAdd}
            >
              {creating ? 'Adding…' : 'Add channel'}
            </button>
          </div>
        )}
      </div>
    </div>
  );
}
