
---

# CLI Reminder Tool 🦀

A fast and simple command-line reminder application written in Rust.
Add, list, remove, and (future) update reminders right from your terminal, with persistent storage in a local file.

---

## ✨ Features

* 📅 **Add reminders** with title, description, due date, and time
* 📋 **List reminders** in a clean table format
* 🗑️ **Remove reminders** by ID
* 🔄 **Update reminders** (planned)
* 💾 **Persistent JSON storage** for reminders

---

## 📦 Installation

### 1️⃣ Clone the repository

```bash
git clone https://github.com/PauloVieira-1/cli-reminder-tool.git
cd cli-reminder-tool
```

### 2️⃣ Build the project

```bash
cargo build --release
```

### 3️⃣ Run the binary

```bash
./target/release/cli-reminder-tool
```

*(Optional: Move binary to PATH for global access)*

```bash
cp ./target/release/cli-reminder-tool /usr/local/bin/reminder
```

---

## 🚀 Usage

### **Add a reminder**

```bash
reminder add <title> <description> <due_date:YYYY-MM-DD> <time:HH:MM>
```

**Example:**

```bash
reminder add "Doctor" "Annual check-up" 2025-08-05 09:00
```

---

### **List all reminders**

```bash
reminder list
```

This displays all reminders with:

* **ID** (used for removal/update)
* **Title**
* **Description**
* **Due date**
* **Timestamp** (when added)

---

### **Remove a reminder**

```bash
reminder remove <id>
```

**Example:**

```bash
reminder remove 12345
```

---

### **Update a reminder** *(planned)*

```bash
reminder update <id> <title> <description> <due_date:YYYY-MM-DD>
```

**Example:**

```bash
reminder update 12345 "Dentist" "Check-up rescheduled" 2025-08-10
```

---
### **Watch a reminder**

# Must be done in order to recieve notifictaions 

```bash
reminder watch
```

## ⚙️ How It Works

* Reminders are stored in a JSON file (default: `reminders.json`) via the `data_manager` module.
* IDs are randomly generated.
* Input date/time format must be `YYYY-MM-DD HH:MM`.

---

## 📂 Project Structure

```
cli-reminder-tool/
│── src/
│   ├── main.rs         # Entry point, command handling
│   ├── reminder.rs     # Reminder struct & command types
│   ├── data_manager.rs # Read/write reminders to file
│   ├── timer.rs        # (Future) reminder timing functionality
│── Cargo.toml          # Rust dependencies
│── README.md           # Documentation
```

---

## 🛠️ Built With

* **Rust** (stable)
* **Clap** (optional in future for CLI parsing)
* **Serde + JSON** for data storage
* **Rand** for ID generation

---

