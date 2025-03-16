def remove(core, packages):
    for package in packages:
        core.remove_package(package)