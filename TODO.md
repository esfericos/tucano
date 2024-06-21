# High-level

- Distribute control-plane through multiple controller nodes (we currently only
  support one controlling node per cluster).

# Mid-level

- Sysadmin notifier.
- Persist node states across crashes.
- Support more service redeployment policies.
- Add correlation IDs and error correlation IDs.

# Refactors

- Rename "deploy_instance" operation to "start_instance". This way, we keep it
  consistent with, e.g., the instance state machine.
