def search(core, query, repo):
    results = core.search_packages(query, repo)
    # print_search_results(results)