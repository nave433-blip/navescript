import os
import sys

# TODOLIST.py - Task Tracking and Missing Feature Manager for navescript
# This script manages the implementation progress of features listed in navescript/TO DO LIST.py.

import os
import sys

# TODOLIST.py - Task Tracking and Missing Feature Manager for navescript

class TaskManager:
    def __init__(self, target_file):
        self.target_file = target_file
        self.tasks = []
        self._load_tasks()

    def _load_tasks(self):
        if os.path.exists(self.target_file):
            with open(self.target_file, 'r', encoding='utf-8') as f:
                self.tasks = f.readlines()
        else:
            print(f"Error: {self.target_file} not found.")

    def list_tasks(self, status_filter=None):
        try:
            print(f"--- Task List {'(' + status_filter + ')' if status_filter else ''} ---")
            for i, line in enumerate(self.tasks):
                if status_filter:
                    if status_filter in line:
                        print(f"{i}: {line.strip()}")
                else:
                    print(f"{i}: {line.strip()}")
        except BrokenPipeError:
            sys.stderr.close()

    def list_must_have(self):
        print("--- Must-Have Features (Production Priority) ---")
        for i, line in enumerate(self.tasks):
            # Based on the criteria observed in the file, 'CRITICAL' tasks are must-haves
            if "CRITICAL" in line.upper():
                print(f"{i}: {line.strip()}")

    def update_task(self, index, new_status):
        if 0 <= index < len(self.tasks):
            # Simple status replacement logic
            line = self.tasks[index]
            if "🔴" in line:
                self.tasks[index] = line.replace("🔴", new_status)
            elif "🟡" in line:
                self.tasks[index] = line.replace("🟡", new_status)
            
            with open(self.target_file, 'w', encoding='utf-8') as f:
                f.writelines(self.tasks)
            print(f"Task {index} updated.")
        else:
            print("Invalid index.")

    def run(self):
        if len(sys.argv) < 2:
            print("Usage: todolist.py [list|update|musthave] [args]")
            return

        command = sys.argv[1]
        if command == "list":
            self.list_tasks(sys.argv[2] if len(sys.argv) > 2 else None)
        elif command == "musthave":
            self.list_must_have()
        elif command == "update":
            if len(sys.argv) > 3:
                self.update_task(int(sys.argv[2]), sys.argv[3])
            else:
                print("Usage: todolist.py update [index] [status_icon]")

if __name__ == "__main__":
    manager = TaskManager("navescript/TO DO LIST.py")
    manager.run()
