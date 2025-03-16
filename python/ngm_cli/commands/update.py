def update(core, packages):
    if packages:
        for package in packages:
            core.update_package(package)
    else:
        core.update_all_packages()