import sys
from rich.console import Console
from rich.panel import Panel
from rich.prompt import Confirm

def install(core, packages, dry_run):
    console = Console()
    for package in packages:
        try:
            deps = core.resolve_dependencies(package)
            
            console.print(Panel.fit(
                f"Dependencies for {package}",
                expand=False
            ))
            # print_dependencies(console, deps)
            
            if not Confirm.ask("Proceed with installation?"):
                console.print("[yellow]Installation cancelled[/yellow]")
                continue
            
            if dry_run:
                console.print("[green]Dry run complete. No packages installed.[/green]")
                continue
            
            # downloads = core.prepare_downloads(deps)
            # progress_tracker = progress.ProgressTracker(console)
            # progress_tracker.start_downloads(downloads)
            # core.install_packages(deps)
            console.print(f"[green]Successfully installed {package}[/green]")
            
        except Exception as e:
            console.print(f"[red]Error installing {package}: {str(e)}[/red]", err=True)