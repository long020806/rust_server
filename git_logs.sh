#!/bin/bash

# 初始化总计数量
total_added=0
total_removed=0
total_lines=0

# 获取当前目录下所有Git仓库文件夹
git_repos=$(find . -type d -name ".git")

for repo in $git_repos; do
	repo_dir=$(dirname "$repo")
	cd "$repo_dir" || continue

	this_year=$(date +'%Y')
	start_date="${this_year}-01-01"
	end_date="${this_year}-12-31"

	result = $(git log --author="long020806" --since="$start_date" --until="$end_date" --pretty=tformat: --numstat | awk -v add=0 -v subs=0 '{add += $1;subs+=$2} END {printf "%s;%s;%s", add, subs, add - subs }')
	IFS=';' read -r added removed lines <<< "$result"

	total_added=$((total_added + added))
	total_removed=$((total_removed + removed))
	total_lines=$((total_lines + lines))

	cd ..
done

echo "Total Added lines: $total_added"
echo "Total Removed lines: $total_removed"
echo "Total Lines: $total_lines"
