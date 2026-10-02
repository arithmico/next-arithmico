# Target Platforms

Arithmico is an accessible scientific calculator designed primarily for students, with tailored support for visually impaired and blind users via assistive technologies.

When establishing the platform strategy, we identified the following core requirements:
1. **Zero-Friction Access:** Users should be able to launch and use the application immediately in standard browser environments without administrative privileges or installation steps.
2. **Cross-Platform Compatibility:** The software must function reliably across diverse operating systems used in schools and universities (Windows, macOS, Linux, ChromeOS, iOS, Android).
3. **Offline Usability:** The tool must function completely offline without active internet connectivity to support strict school examination environments and locations with limited connectivity.
4. **Accessibility Integration:** Deep compatibility with OS screen readers (NVDA, JAWS, VoiceOver, Orca) and Braille devices.

## Decision

We decided to build Arithmico as a **Web Application** first, using standard web technologies.

To fulfill offline examination requirements while retaining a seamless user experience, we distribute Arithmico in two ways:
1. **Primary Distribution:** Web application hosted online for instant browser access.
2. **Offline Distribution:** The web application packaged using **Tauri** as a lightweight native desktop wrapper for offline and examination environments.

## Alternatives Considered

### Pure Native Desktop Application
* **Pros:** Direct OS integration, offline by default.
* **Cons:**
  * **Installation Barrier:** Requires installation and administrative privileges, which are often restricted on school computers and lab workstations.
  * **Complex Update Management:** Managing client-side updates, platform-specific installers, auto-update pipelines, and binary patches across Windows, macOS, and Linux adds significant maintenance overhead compared to deploying a web application.
  * **Limited Reach:** Prevents quick evaluation and usage on devices where software installation is forbidden or unsupported (e.g., ChromeOS, mobile devices).

### Heavyweight Hybrid Desktop Framework (Electron)
* **Pros:** Bundles Chromium, guaranteeing consistent web runtime behavior offline.
* **Cons:** Large binary sizes, high memory consumption, and unnecessary resource overhead compared to lighter alternatives like Tauri.

## Consequences

### Positive
* **Instant Availability:** Web deployment allows users to open and use Arithmico immediately via any modern browser without installation.
* **Flexible Offline Deployments:** Wrapping the core web application in **Tauri** yields lightweight, portable desktop binaries suitable for offline exam environments.
* **Simplified Core Maintenance:** A single core codebase powers both the online web application and the offline desktop wrapper.
* **Accessibility Compatibility:** HTML/ARIA accessibility trees natively integrate with system screen readers across both browser and desktop wrapper environments.

### Negative / Trade-offs
* **Update Synchronization:** Offline desktop distributions wrapped in Tauri require manual update handling or dedicated updater logic, whereas web application deployments update instantly for all online users.
* **Platform Testing:** Requires maintaining build pipelines and testing cross-platform desktop builds (Windows/macOS/Linux) alongside standard web deployment.
