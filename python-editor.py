# python3.13 python-editor.py

import tkinter as tk
from tkinter import filedialog, messagebox


class SimpleEditor(tk.Tk):
    def __init__(self):
        super().__init__()
        self.title("Simple Python Text Editor")
        self.geometry("900x600")

        self.filename = None

        # Text area + scrollbar
        self.text = tk.Text(self, wrap="word", undo=True)
        self.scroll = tk.Scrollbar(self, command=self.text.yview)
        self.text.configure(yscrollcommand=self.scroll.set)

        self.scroll.pack(side="right", fill="y")
        self.text.pack(side="left", fill="both", expand=True)

        self._build_menu()
        self._bind_shortcuts()

    def _build_menu(self):
        menubar = tk.Menu(self)

        file_menu = tk.Menu(menubar, tearoff=0)
        file_menu.add_command(label="New", command=self.new_file, accelerator="Ctrl+N")
        file_menu.add_command(
            label="Open...", command=self.open_file, accelerator="Ctrl+O"
        )
        file_menu.add_command(
            label="Save", command=self.save_file, accelerator="Ctrl+S"
        )
        file_menu.add_command(
            label="Save As...", command=self.save_file_as, accelerator="Ctrl+Shift+S"
        )
        file_menu.add_separator()
        file_menu.add_command(label="Exit", command=self.on_exit)
        menubar.add_cascade(label="File", menu=file_menu)

        edit_menu = tk.Menu(menubar, tearoff=0)
        edit_menu.add_command(
            label="Undo", command=self.text.edit_undo, accelerator="Ctrl+Z"
        )
        edit_menu.add_command(
            label="Redo", command=self.text.edit_redo, accelerator="Ctrl+Y"
        )
        edit_menu.add_separator()
        edit_menu.add_command(
            label="Cut",
            command=lambda: self.event_generate("<<Cut>>"),
            accelerator="Ctrl+X",
        )
        edit_menu.add_command(
            label="Copy",
            command=lambda: self.event_generate("<<Copy>>"),
            accelerator="Ctrl+C",
        )
        edit_menu.add_command(
            label="Paste",
            command=lambda: self.event_generate("<<Paste>>"),
            accelerator="Ctrl+V",
        )
        menubar.add_cascade(label="Edit", menu=edit_menu)

        self.config(menu=menubar)

    def _bind_shortcuts(self):
        self.bind("<Control-n>", lambda e: self.new_file())
        self.bind("<Control-o>", lambda e: self.open_file())
        self.bind("<Control-s>", lambda e: self.save_file())
        self.bind(
            "<Control-S>", lambda e: self.save_file_as()
        )  # Ctrl+Shift+S often maps to Control+S (capital S)

        # Optional: confirm close if modified
        self.protocol("WM_DELETE_WINDOW", self.on_exit)

    def _set_title(self):
        name = self.filename if self.filename else "Untitled"
        self.title(f"{name} - Simple Python Text Editor")

    def new_file(self):
        if self._confirm_discard_if_modified() is False:
            return
        self.filename = None
        self.text.delete("1.0", "end")
        self.text.edit_modified(False)
        self._set_title()

    def open_file(self):
        if self._confirm_discard_if_modified() is False:
            return
        path = filedialog.askopenfilename(
            filetypes=[("Text files", "*.txt"), ("All files", "*.*")]
        )
        if not path:
            return
        try:
            with open(path, "r", encoding="utf-8") as f:
                content = f.read()
            self.text.delete("1.0", "end")
            self.text.insert("1.0", content)
            self.filename = path
            self.text.edit_modified(False)
            self._set_title()
        except Exception as ex:
            messagebox.showerror("Open failed", str(ex))

    def save_file(self):
        if not self.filename:
            return self.save_file_as()
        try:
            content = self.text.get("1.0", "end-1c")
            with open(self.filename, "w", encoding="utf-8") as f:
                f.write(content)
            self.text.edit_modified(False)
            self._set_title()
        except Exception as ex:
            messagebox.showerror("Save failed", str(ex))

    def save_file_as(self):
        path = filedialog.asksaveasfilename(
            defaultextension=".txt",
            filetypes=[("Text files", "*.txt"), ("All files", "*.*")],
        )
        if not path:
            return
        self.filename = path
        self.save_file()

    def _confirm_discard_if_modified(self):
        # Tkinter tracks modification state with edit_modified()
        if self.text.edit_modified():
            answer = messagebox.askyesnocancel(
                "Unsaved changes", "You have unsaved changes. Save before continuing?"
            )
            if answer is None:  # Cancel
                return False
            if answer is True:  # Yes -> Save
                self.save_file()
                # if save was canceled in save_as, still modified
                return not self.text.edit_modified()
            return True  # No -> discard
        return True

    def on_exit(self):
        if self._confirm_discard_if_modified() is False:
            return
        self.destroy()


if __name__ == "__main__":
    app = SimpleEditor()
    app._set_title()
    app.mainloop()
