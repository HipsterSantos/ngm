import argparse
import sys
from typing import List

from ngm_cli import commands
from ngm_cli.utils import load_core

def main():
    parser = argparse.ArgumentParser(description='ngm - Next Generation Package Manager')
    subparsers = parser.add_subparsers(dest='command', required=True)

    # Install command
    install_parser = subparsers.add_parser('install', help='Install packages')
    install_parser.add_argument('packages', nargs='+', help='Packages to install')
    install_parser.add_argument('--dry-run', action='store_true', help='Preview installation without actual download')

    # Search command
    search_parser = subparsers.add_parser('search', help='Search for packages')
    search_parser.add_argument('query', help='Search query')
    search_parser.add_argument('--repo', help='Limit search to specific repository')

    # Update command
    update_parser = subparsers.add_parser('update', help='Update packages')
    update_parser.add_argument('packages', nargs='*', help='Packages to update')

    # Remove command
    remove_parser = subparsers.add_parser('remove', help='Remove packages')
    remove_parser.add_argument('packages', nargs='+', help='Packages to remove')

    # List command
    list_parser = subparsers.add_parser('list', help='List installed packages')

    # Show command
    show_parser = subparsers.add_parser('show', help='Show package information')
    show_parser.add_argument('package', help='Package name')

    # Dependencies command
    deps_parser = subparsers.add_parser('deps', help='Show package dependencies')
    deps_parser.add_argument('package', help='Package name')

    args = parser.parse_args()

    core = load_core()

    try:
        if args.command == 'install':
            commands.install(core, args.packages, args.dry_run)
        elif args.command == 'search':
            commands.search(core, args.query, args.repo)
        elif args.command == 'update':
            commands.update(core, args.packages)
        elif args.command == 'remove':
            commands.remove(core, args.packages)
        elif args.command == 'list':
            commands.list_installed(core)
        elif args.command == 'show':
            commands.show(core, args.package)
        elif args.command == 'deps':
            commands.show_deps(core, args.package)
    except Exception as e:
        print(f"Error: {str(e)}", file=sys.stderr)
        sys.exit(1)

if __name__ == '__main__':
    main()