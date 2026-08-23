# 🩺 Pill Detect

**AI-powered medical assistant for medicine information, prescription analysis, symptom guidance, and drug interaction checking.**

[![Rust](https://img.shields.io/badge/Rust-2024-orange?logo=rust)](https://www.rust-lang.org/)
[![Dioxus](https://img.shields.io/badge/Dioxus-0.7.10-blue)](https://dioxuslabs.com/)
[![License](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)

**Live Demo:** https://sonickajadeesh.github.io/pill-detect/

---

## Overview

**Pill Detect** is a privacy-focused, browser-based AI medical assistant designed to help users organize patient information and access useful medication-related guidance.

The application combines a **Rust + Dioxus frontend** with **Google Gemini** to provide:

- 💊 Medicine identification and information
- 📋 Prescription image analysis
- 💬 Symptom and health guidance
- ⚠️ Drug interaction checking
- 👤 Patient profile management
- 📝 Prescription management
- 🔎 Medicine search history
- 💾 Persistent browser-based storage

The application is designed as an informational assistant and **is not a replacement for a qualified healthcare professional.**

---

## ✨ Features

### 👤 Patient Management

Create and manage patient profiles containing:

- First and last name
- Date of birth
- Sex
- Blood group
- Height
- Weight
- Allergies
- Existing medical conditions

Patients can be **created, edited, viewed, and deleted** from the main dashboard.

Patient records and associated histories are stored locally in the browser using `localStorage`.

---

### 💊 Medicine Information

Search for a medicine and retrieve:

- Product name
- Generic name
- Uses
- Typical dosage information
- Common side effects
- Warnings
- Prescription requirement

Medicine identification and research are handled through Gemini-powered prompts with instructions to use web search and reliable sources rather than guessing.

---

### 📋 Prescription Analysis

Upload a prescription image and let the AI extract prescribed medications.

The analysis can identify:

- Medicine name
- Strength
- Dosage
- Duration
- Instructions

The extracted prescription can then be saved to the patient's profile for future reference.

---

### 💬 Symptom Guidance

The symptom guidance feature provides conversational medical information while maintaining conversation history for each patient.

The AI is instructed to:

- Respond naturally
- Use previous conversation context
- Avoid diagnosing
- Avoid inventing medical information
- Clearly communicate uncertain information
- Keep responses concise and medically responsible

---

### ⚠️ Drug Interaction Checker

Check a patient's medicines for clinically relevant safety concerns.

The interaction checker considers:

- Drug-to-drug interactions
- Recorded allergies
- Existing medical conditions
- Interaction severity
- Potential effects
- Recommended action

Interaction searches can also be stored in the patient's history.

---

## 🧠 AI Architecture

Pill Detect uses **Google Gemini** for its AI-powered functionality.

The application communicates directly with the Gemini API for both text and image-based requests. Prescription images are encoded and sent as inline image data for analysis.

The current implementation uses:

```text
Dioxus Web App
      │
      ├── Patient Management
      │
      ├── Medicine Information
      │
      ├── Prescription Analysis
      │
      ├── Symptom Guidance
      │
      └── Drug Interaction Checker
              │
              ▼
        Google Gemini API
```

---

## 🔐 Privacy & Data Storage

Pill Detect currently uses browser-local storage rather than a remote patient database.

Patient profiles, medicine search history, conversations, prescriptions, and interaction history are stored using browser `localStorage`.

The Gemini API key is also stored locally in the browser after the user enters it. Users can clear or replace the stored key from the application.

> **Important:** Do not use this application as a secure electronic health record system or enter sensitive medical information into an environment you do not trust.

---

## 🛠️ Tech Stack

| Technology             | Purpose                          |
| ---------------------- | -------------------------------- |
| **Rust**               | Application programming language |
| **Dioxus 0.7.10**      | UI framework                     |
| **Dioxus Router**      | Client-side routing              |
| **Tailwind CSS**       | Styling                          |
| **Google Gemini API**  | AI and image analysis            |
| **Serde / Serde JSON** | Data serialization               |
| **Reqwest**            | HTTP requests                    |
| **Gloo Storage**       | Browser local storage            |
| **WebAssembly**        | Web application runtime          |
| **GitHub Pages**       | Deployment                       |

The project currently targets the Dioxus web platform and uses Dioxus's router for patient-specific application routes.

---

## 📁 Project Structure

```text
pill-detect/
├── .github/
├── assets/
├── src/
│   ├── components/
│   │   ├── dashboard.rs
│   │   ├── footer.rs
│   │   ├── guidance.rs
│   │   ├── homepage.rs
│   │   ├── information.rs
│   │   ├── interaction.rs
│   │   ├── navbar.rs
│   │   └── prescription.rs
│   │
│   ├── modules/
│   │   ├── api.rs
│   │   ├── database.rs
│   │   ├── prompts.rs
│   │   └── utilities.rs
│   │
│   └── main.rs
│
├── Cargo.toml
├── Cargo.lock
├── Dioxus.toml
├── LICENSE
├── clippy.toml
└── tailwind.css
```

---

## 🚀 Getting Started

### Prerequisites

Install:

- [Rust](https://www.rust-lang.org/tools/install)
- [Dioxus CLI](https://dioxuslabs.com/learn/0.7/getting_started/)

Install the Dioxus CLI with:

```bash
cargo install dioxus-cli
```

Clone the repository:

```bash
git clone https://github.com/sonickajadeesh/pill-detect.git
cd pill-detect
```

Install/build the project dependencies:

```bash
cargo check
```

Start the Dioxus development server:

```bash
dx serve
```

Then open the local address shown by Dioxus.

---

## 🔑 Gemini API Key

Pill Detect requires a **Google Gemini API key** for its AI features.

When the application starts, it asks the user for a Gemini API key if one has not already been stored in browser storage.

The key is saved locally in the browser and reused for subsequent requests.

You can obtain a Gemini API key from:

https://aistudio.google.com/apikey

> **Security note:** This project currently uses the Gemini API directly from the client-side application. For production healthcare use, API requests should be routed through a secure backend so that API credentials are not exposed to clients.

---

## 🧭 Application Routes

The application currently provides the following routes:

| Route                         | Purpose                               |
| ----------------------------- | ------------------------------------- |
| `/`                           | Patient registration and patient list |
| `/:patient_id/`               | Patient dashboard                     |
| `/:patient_id/guidance`       | Symptom guidance                      |
| `/:patient_id/information/`   | Medicine information                  |
| `/:patient_id/interaction/`   | Drug interaction checker              |
| `/:patient_id/prescriptions/` | Prescription analysis and management  |

These routes are defined using the Dioxus router.

---

## 🖥️ Deployment

The project is configured for a Dioxus web build with the GitHub Pages base path:

```text
pill-detect
```

The configured live application is:

**https://sonickajadeesh.github.io/pill-detect/**

The repository also contains GitHub Actions configuration for the project.

---

## ⚠️ Medical Disclaimer

Pill Detect is an **AI-assisted informational application**.

It does not provide professional medical diagnosis, treatment, or emergency medical advice.

AI-generated information may be incomplete, inaccurate, outdated, or inappropriate for a particular patient.

Always:

- Verify medication information with reliable medical sources.
- Follow instructions from qualified healthcare professionals.
- Consult a doctor or pharmacist before changing medication.
- Seek professional medical care for serious or worsening symptoms.
- Never rely solely on AI-generated information for medical decisions.
