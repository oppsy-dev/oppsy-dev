<div style="text-align: center; margin-bottom: 1.5rem;">
  <img src="logo.svg" alt="OPPSY" width="48">
</div>

# Introduction

OPPSY is an open-source vulnerability management platform. It watches your project's dependency lock files, continuously checks them against the [OSV database](https://osv.dev), and notifies you whenever a vulnerability is found or updated. The goal is simple: keep your team informed about security issues in the libraries you depend on, without requiring manual effort.

## Core concepts

There are three things you work with in OPPSY:

**Workspace** — a logical container for a project. Each workspace holds the dependency manifests you want to monitor and the notification channels that should receive alerts for that project.

**Manifest** — a dependency lock file uploaded into a workspace. OPPSY parses it, identifies every package it declares, and matches those packages against the OSV vulnerability database. You can upload as many manifests as you like into a single workspace — useful when a project spans multiple language ecosystems.

**Notification channel** — a configured destination that receives vulnerability alerts. A channel can be a webhook endpoint, a Discord channel, or an email address. Channels are created independently and then linked to one or more workspaces.

## How it works

1. Create a workspace for your project.
2. Upload one or more dependency lock files into that workspace.
3. Create a notification channel and link it to the workspace.
4. OPPSY scans the manifests, matches packages against OSV, and delivers alerts to your channel.
5. In the background, OPPSY keeps the OSV database in sync. When a new vulnerability is published or an existing one is updated, any affected workspaces are notified automatically.
