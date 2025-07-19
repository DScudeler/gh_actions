# GitHub Actions Setup Guide

This guide explains how to set up automated testing and deployment for the WASM Task Manager project using GitHub Actions.

## Overview

The project includes two main workflows:
1. **CI (Continuous Integration)** - Runs tests, linting, and builds on every push/PR
2. **CD (Continuous Deployment)** - Automatically deploys to GitHub Pages on main branch updates

## Quick Setup

### Step 1: Create Workflow Directory
```bash
mkdir -p .github/workflows
```

### Step 2: Copy Workflow Files
Copy the following files to your `.github/workflows/` directory:
- `ci-test-workflow.yml` → `.github/workflows/ci.yml`
- `cd-deploy-workflow.yml` → `.github/workflows/deploy.yml`

### Step 3: Enable GitHub Pages
1. Go to your repository Settings
2. Navigate to "Pages" in the sidebar
3. Under "Source", select "GitHub Actions"
4. Save the configuration

## Workflow Details

### CI Workflow (`ci.yml`)

**Triggers:**
- Push to `main` or `develop` branches
- Pull requests to `main` branch

**Features:**
- ✅ Rust toolchain setup with stable channel
- ✅ Cargo caching for faster builds
- ✅ Code formatting checks with `cargo fmt`
- ✅ Linting with `cargo clippy`
- ✅ Unit tests with `cargo test`
- ✅ Benchmark compilation verification
- ✅ WASM package building with `wasm-pack`
- ✅ Artifact upload for built WASM package

**Artifacts:** The WASM package is uploaded and retained for 30 days.

### CD Workflow (`deploy.yml`)

**Triggers:**
- Push to `main` branch (automatic)
- Manual trigger via workflow dispatch

**Features:**
- ✅ Builds WASM package for production
- ✅ Sets up GitHub Pages configuration
- ✅ Deploys to GitHub Pages automatically
- ✅ Concurrent deployment protection

**Permissions Required:**
- `contents: read` - Read repository content
- `pages: write` - Write to GitHub Pages
- `id-token: write` - Authentication token

## Repository Configuration

### Required Secrets
No additional secrets are required for basic functionality. GitHub automatically provides the necessary tokens.

### Branch Protection (Recommended)
Consider setting up branch protection rules:

1. Go to Settings → Branches
2. Add rule for `main` branch
3. Enable:
   - Require status checks to pass before merging
   - Require branches to be up to date before merging
   - Include administrators in restrictions

### Status Checks
After setting up workflows, you can require the following status checks:
- `Test Suite` (from CI workflow)
- `Build WASM Application` (from CD workflow)

## Local Development Integration

The workflows integrate with your existing local development setup:

- **Build Script**: Uses the existing `build-wasm.sh`
- **Rust Toolchain**: Respects `rust-toolchain.toml` configuration
- **Dependencies**: Uses `Cargo.toml` for dependency management
- **Tests**: Runs all tests defined in the `tests/` directory

## Deployment URL

After successful deployment, your WASM Task Manager will be available at:
```
https://{username}.github.io/{repository-name}/
```

For this repository: `https://DScudeler.github.io/gh_actions/`

## Troubleshooting

### Common Issues

**Build Failures:**
- Ensure `build-wasm.sh` is executable: `chmod +x build-wasm.sh`
- Check that all dependencies are properly declared in `Cargo.toml`

**Deployment Issues:**
- Verify GitHub Pages is enabled in repository settings
- Check that the `pages` environment exists and is configured correctly
- Ensure the repository has public visibility or GitHub Pro/Team plan

**Permission Errors:**
- Verify the workflow has proper permissions (see workflow files)
- Check repository settings for Actions permissions

### Viewing Logs
1. Go to the "Actions" tab in your repository
2. Click on any workflow run
3. Expand job steps to view detailed logs

## Customization

### Adding More Environments
To deploy to staging/production environments:
1. Create additional workflow files
2. Use different trigger conditions
3. Set up environment-specific secrets

### Custom Build Steps
To add custom build steps:
1. Modify the workflow files
2. Add additional `run` steps or actions
3. Update caching strategies if needed

### Notifications
To add notifications on build success/failure:
```yaml
- name: Notify on failure
  if: failure()
  uses: actions/github-script@v6
  with:
    script: |
      github.rest.issues.createComment({
        issue_number: context.issue.number,
        owner: context.repo.owner,
        repo: context.repo.repo,
        body: '❌ Build failed! Check the logs for details.'
      });
```

## Security Considerations

- Workflows run with limited permissions by default
- No secrets are exposed in build logs
- Dependencies are cached securely
- Deployment uses official GitHub Actions

## Performance Tips

- **Caching**: Both workflows include Cargo caching for faster builds
- **Parallel Jobs**: Consider splitting tests and builds into separate jobs for faster feedback
- **Artifact Cleanup**: Old artifacts are automatically cleaned up after 30 days

## Next Steps

1. Set up the workflows as described above
2. Make a test commit to trigger the CI pipeline
3. Verify the deployment works correctly
4. Consider adding additional quality gates (security scanning, performance tests)
5. Set up branch protection rules for production safety

## Support

For issues with GitHub Actions:
- Check the [GitHub Actions documentation](https://docs.github.com/en/actions)
- Review workflow run logs in the Actions tab
- Consult the [GitHub Community Forum](https://github.community)