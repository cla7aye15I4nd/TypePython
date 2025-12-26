#!/usr/bin/env python3
"""Generate coverage report markdown from cargo llvm-cov output."""

import re
import sys


def parse_coverage_report(report_path):
    """Parse the coverage report text file."""
    with open(report_path, 'r') as f:
        report = f.read()

    lines = report.strip().split('\n')
    coverage_lines = []
    total_line = None
    in_coverage_section = False

    for line in lines:
        # Skip header and look for the data section
        if 'Filename' in line and 'Regions' in line:
            in_coverage_section = True
            continue

        # Skip the separator line
        if in_coverage_section and line.strip().startswith('---'):
            continue

        if in_coverage_section and line.strip():
            # Check if this is the TOTAL line
            if line.strip().startswith('TOTAL'):
                total_line = line
            else:
                # Parse coverage lines (skip empty lines)
                coverage_lines.append(line)

    return coverage_lines, total_line


def create_markdown_table(coverage_lines, total_line):
    """Create a markdown table from coverage data."""
    md_table = "## Code Coverage Report\n\n"
    md_table += "| File | Regions | Miss | Cover | Functions | Miss | Exec | Lines | Miss | Cover |\n"
    md_table += "|------|---------|------|-------|-----------|------|------|-------|------|-------|\n"

    # Add up to 20 files
    for line in coverage_lines[:20]:
        parts = line.split()
        if len(parts) >= 10:
            filename = parts[0]
            regions = parts[1]
            regions_miss = parts[2]
            regions_cover = parts[3]
            functions = parts[4]
            functions_miss = parts[5]
            functions_exec = parts[6]
            lines = parts[7]
            lines_miss = parts[8]
            lines_cover = parts[9]

            # Shorten long paths
            if len(filename) > 45:
                filename = "..." + filename[-42:]

            md_table += f"| {filename} | {regions} | {regions_miss} | {regions_cover} | {functions} | {functions_miss} | {functions_exec} | {lines} | {lines_miss} | {lines_cover} |\n"

    # Add total line
    if total_line:
        parts = total_line.split()
        if len(parts) >= 10:
            md_table += f"| **TOTAL** | **{parts[1]}** | **{parts[2]}** | **{parts[3]}** | **{parts[4]}** | **{parts[5]}** | **{parts[6]}** | **{parts[7]}** | **{parts[8]}** | **{parts[9]}** |\n"

    md_table += "\n*Generated from latest CI run*\n"

    return md_table


def update_readme(readme_path, coverage_markdown):
    """Update README with coverage report."""
    with open(readme_path, 'r') as f:
        readme = f.read()

    # Check if coverage section exists
    if '## Code Coverage Report' in readme:
        # Replace existing coverage section
        pattern = r'## Code Coverage Report.*?(?=\n## |\Z)'
        readme = re.sub(pattern, coverage_markdown.rstrip(), readme, flags=re.DOTALL)
    else:
        # Add coverage section after CI badge
        badge_pattern = r'(\[\!\[CI\].*?\))\n\n'
        replacement = r'\1\n\n' + coverage_markdown + '\n\n'
        readme = re.sub(badge_pattern, replacement, readme)

    # Write updated README
    with open(readme_path, 'w') as f:
        f.write(readme)

    print("README updated with coverage report")


def main():
    if len(sys.argv) < 2:
        print("Usage: generate_coverage_report.py <coverage-report.txt> [README.md]")
        sys.exit(1)

    report_path = sys.argv[1]
    readme_path = sys.argv[2] if len(sys.argv) > 2 else 'README.md'

    # Parse coverage report
    coverage_lines, total_line = parse_coverage_report(report_path)

    # Create markdown table
    coverage_markdown = create_markdown_table(coverage_lines, total_line)

    # Save to file
    with open('coverage-summary.md', 'w') as f:
        f.write(coverage_markdown)
    print("Coverage summary saved to coverage-summary.md")

    # Update README
    update_readme(readme_path, coverage_markdown)


if __name__ == '__main__':
    main()
