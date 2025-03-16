import time
import threading
from rich.progress import Progress, BarColumn, TextColumn, TimeRemainingColumn

class ProgressTracker:
    def __init__(self, console):
        self.progress = Progress(
            TextColumn("[progress.description]{task.description}"),
            BarColumn(),
            TextColumn("[progress.percentage]{task.percentage:>3.0f}%"),
            TimeRemainingColumn(),
        )
        self.lock = threading.Lock()
        self.active = False
        self.console = console

    def start_downloads(self, downloads):
        with self.progress:
            self.active = True
            tasks = {}
            for download in downloads:
                task_id = self.progress.add_task(
                    download['package_name'], 
                    total=download['size']
                )
                tasks[download['id']] = task_id
            
            # Simulate download progress
            while self.active:
                time.sleep(0.1)
                with self.lock:
                    # Update progress from Rust core
                    self._update_progress(tasks)
    
    def _update_progress(self, tasks):
        # This would interface with Rust to get current download status
        # For demonstration, we'll simulate progress
        for task_id, progress in tasks.items():
            current = min(progress['current'] + 1024 * 1024, progress['total'])
            self.progress.update(task_id, completed=current)
            if current >= progress['total']:
                self.progress.update(task_id, completed=progress['total'])
                self.progress.stop_task(task_id)
    
    def finish_download(self, download_id):
        with self.lock:
            self.active = False