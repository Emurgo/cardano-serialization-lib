#!/usr/bin/env node

/**
 * Cardano Serialization Library Build Helper
 * 
 * Supports multiple actions:
 * - build: Build for specific target/variant
 * - publish: Build and publish a single package  
 * - publish-all: Run tests + build and publish ALL packages + publish Rust crate
 * 
 * All parameters are REQUIRED - no defaults provided to prevent accidental builds.
 */

const { Command } = require('commander');
const { execSync } = require('child_process');
const fs = require('fs');

const program = new Command();

program
  .name('build-helper')
  .description('Cardano Serialization Library Build Helper\n\nExamples:\n  build-helper build --target browser --variant normal --gc false\n  build-helper publish --target nodejs --variant normal --gc true --env beta\n  build-helper publish-all --env prod')
  .version('1.0.0');

// Build command
program
  .command('build')
  .description('Build for specific target/variant\n\nExample: build --target browser --variant normal --gc false')
  .requiredOption('-t, --target <target>', 'Target platform (nodejs|browser|web)', (value) => {
    if (!['nodejs', 'browser', 'web'].includes(value)) {
      throw new Error('Target must be: nodejs, browser, or web');
    }
    return value;
  })
  .requiredOption('-v, --variant <variant>', 'Build variant (normal|inlined|asm)', (value) => {
    if (!['normal', 'inlined', 'asm'].includes(value)) {
      throw new Error('Variant must be: normal, inlined, or asm');
    }
    return value;
  })
  .requiredOption('-g, --gc <gc>', 'Enable garbage collection (true|false)', (value) => {
    if (value !== 'true' && value !== 'false') {
      throw new Error('GC must be: true or false');
    }
    return value === 'true';
  })
  .action((options) => {
    buildRust(options.target, options.variant, options.gc);
    console.log(`\n✅ Build completed successfully!`);
  });

// Publish command  
program
  .command('publish')
  .description('Build and publish a single package\n\nExample: publish --target nodejs --variant normal --gc true --env beta')
  .requiredOption('-t, --target <target>', 'Target platform (nodejs|browser|web)', (value) => {
    if (!['nodejs', 'browser', 'web'].includes(value)) {
      throw new Error('Target must be: nodejs, browser, or web');
    }
    return value;
  })
  .requiredOption('-v, --variant <variant>', 'Build variant (normal|inlined|asm)', (value) => {
    if (!['normal', 'inlined', 'asm'].includes(value)) {
      throw new Error('Variant must be: normal, inlined, or asm');
    }
    return value;
  })
  .requiredOption('-g, --gc <gc>', 'Enable garbage collection (true|false)', (value) => {
    if (value !== 'true' && value !== 'false') {
      throw new Error('GC must be: true or false');
    }
    return value === 'true';
  })
  .requiredOption('-e, --env <env>', 'Environment (prod|beta)', (value) => {
    if (!['prod', 'beta'].includes(value)) {
      throw new Error('Environment must be: prod or beta');
    }
    return value;
  })
  .action((options) => {
    publishPackage(options.target, options.variant, options.gc, options.env);
    console.log(`\n✅ Publish completed successfully!`);
  });

// Publish-all command
program
  .command('publish-all')
  .description('Run tests + build and publish ALL packages + publish Rust crate\n\nBuilds all 10 variants: nodejs/browser/web × normal/inlined/asm × gc/no-gc\n\nExample: publish-all --env prod')
  .requiredOption('-e, --env <env>', 'Environment (prod|beta)', (value) => {
    if (!['prod', 'beta'].includes(value)) {
      throw new Error('Environment must be: prod or beta');
    }
    return value;
  })
  .action((options) => {
    publishAllPackages(options.env);
  });

// Test-publish command
program
  .command('test-publish')
  .description('Test the publishing pipeline without actually publishing\n\nRuns all build steps and validates packages using npm pack and --dry-run\n\nExample: test-publish --env beta')
  .requiredOption('-e, --env <env>', 'Environment (prod|beta)', (value) => {
    if (!['prod', 'beta'].includes(value)) {
      throw new Error('Environment must be: prod or beta');
    }
    return value;
  })
  .action((options) => {
    testPublishAllPackages(options.env);
  });

function run(command, description) {
  console.log(`\n📦 ${description}...`);
  try {
    execSync(command, { stdio: 'inherit', cwd: process.cwd() });
  } catch (error) {
    console.error(`❌ Failed: ${description}`);
    process.exit(1);
  }
}

function buildRust(target, variant, gc) {
  // Clean
  run('npx rimraf ./rust/pkg', 'Cleaning pkg directory');
  
  // Build
  const buildCmd = gc 
    ? `cd rust && WASM_BINDGEN_WEAKREF=1 wasm-pack build --target=${target}`
    : `cd rust && wasm-pack build --target=${target}`;
  
  run(buildCmd, `Building Rust for ${target}${gc ? ' (with GC)' : ''}`);
  
  // Post-build steps based on variant
  run('npm run js:ts-json-gen', 'Generating TypeScript definitions');
  
  if (variant === 'inlined') {
    run('npm run wasm:inline', 'Inlining WASM');
    run('npm run wasm:delete-wasm-files', 'Cleaning up WASM files');
  } else if (variant === 'asm') {
    run('npm run asm:build', 'Building ASM.js version');
  }
  
  run('cd rust && wasm-pack pack', 'Packing Rust build');
  run('npm run js:flowgen', 'Generating Flow types');
}

function checkVersionExists(packageName, version) {
  try {
    const result = execSync(`npm view ${packageName}@${version} version 2>/dev/null`, { 
      encoding: 'utf8', 
      stdio: 'pipe' 
    });
    return result.trim() === version;
  } catch (error) {
    // Package or version doesn't exist
    return false;
  }
}

function getPackageInfo(packageJsonPath) {
  try {
    const packageJson = JSON.parse(fs.readFileSync(packageJsonPath, 'utf8'));
    return {
      name: packageJson.name,
      version: packageJson.version
    };
  } catch (error) {
    console.error(`❌ Failed to read package.json from ${packageJsonPath}`);
    return null;
  }
}

function publishPackage(target, variant, gc, env) {
  const publishTag = env === 'beta' ? '--tag beta' : '';
  
  // Build first
  buildRust(target, variant, gc);

  // Run tests
  run('npm run rust:test', 'Running Rust tests');
  
  // Prepare publish
  run('npm run js:prepublish', 'Preparing for publish');
  
  // Configure package
  const targetSuffix = variant === 'normal' ? target : `${target}-${variant}`;
  const gcSuffix = gc ? ' -gc' : '';
  const helperCmd = `node ./scripts/publish-helper -${targetSuffix}${gcSuffix}`;
  
  run(helperCmd, 'Configuring package for publish');
  
  // Check if version already exists
  const packageInfo = getPackageInfo('./publish/package.json');
  if (packageInfo) {
    console.log(`\n🔍 Checking if ${packageInfo.name}@${packageInfo.version} already exists...`);
    
    if (checkVersionExists(packageInfo.name, packageInfo.version)) {
      console.log(`\n⚠️  WARNING: Package ${packageInfo.name}@${packageInfo.version} already exists on npm!`);
      console.log(`   Please bump the version in package.json before publishing.`);
      console.log(`\n❌ Aborting publish to prevent duplicate version.`);
      process.exit(1);
    } else {
      console.log(`✅ Version ${packageInfo.version} is available for publishing.`);
    }
  }
  
  // Publish
  run(`cd publish && npm publish ${publishTag} --access public`, `Publishing to npm${env === 'beta' ? ' (beta)' : ''}`);
}

function publishSinglePackage(config, env) {
  const { target: configTarget, variant: configVariant, gc: configGc } = config;
  const publishTag = env === 'beta' ? '--tag beta' : '';
  
  // Build first
  buildRust(configTarget, configVariant, configGc);
  
  // Prepare publish
  run('npm run js:prepublish', 'Preparing for publish');
  
  // Configure package
  const targetSuffix = configVariant === 'normal' ? configTarget : `${configTarget}-${configVariant}`;
  const gcSuffix = configGc ? ' -gc' : '';
  const helperCmd = `node ./scripts/publish-helper -${targetSuffix}${gcSuffix}`;
  
  run(helperCmd, 'Configuring package for publish');
  
  // Check if version already exists
  const packageInfo = getPackageInfo('./publish/package.json');
  if (packageInfo) {
    console.log(`\n🔍 Checking if ${packageInfo.name}@${packageInfo.version} already exists...`);
    
    if (checkVersionExists(packageInfo.name, packageInfo.version)) {
      console.log(`\n⚠️  WARNING: Package ${packageInfo.name}@${packageInfo.version} already exists on npm!`);
      console.log(`   Please bump the version in package.json before publishing.`);
      console.log(`\n❌ Aborting publish to prevent duplicate version.`);
      throw new Error(`Version ${packageInfo.version} already exists`);
    } else {
      console.log(`✅ Version ${packageInfo.version} is available for publishing.`);
    }
  }
  
  // Publish
  run(`cd publish && npm publish ${publishTag} --access public`, `Publishing to npm${env === 'beta' ? ' (beta)' : ''}`);
}

function testPublishSinglePackage(config, env) {
  const { target: configTarget, variant: configVariant, gc: configGc } = config;
  const publishTag = env === 'beta' ? '--tag beta' : '';
  
  // Build first
  buildRust(configTarget, configVariant, configGc);
  
  // Prepare publish
  run('npm run js:prepublish', 'Preparing for publish');
  
  // Configure package
  const targetSuffix = configVariant === 'normal' ? configTarget : `${configTarget}-${configVariant}`;
  const gcSuffix = configGc ? ' -gc' : '';
  const helperCmd = `node ./scripts/publish-helper -${targetSuffix}${gcSuffix}`;
  
  run(helperCmd, 'Configuring package for publish');
  
  // Check if version already exists
  const packageInfo = getPackageInfo('./publish/package.json');
  if (packageInfo) {
    console.log(`\n🔍 Checking if ${packageInfo.name}@${packageInfo.version} already exists...`);
    
    if (checkVersionExists(packageInfo.name, packageInfo.version)) {
      console.log(`\n⚠️  Package ${packageInfo.name}@${packageInfo.version} already exists on npm.`);
      console.log(`   In a real publish, this would require a version bump.`);
    } else {
      console.log(`✅ Version ${packageInfo.version} is available for publishing.`);
    }
  }
  
  // Test publish using dry-run and pack
  run(`cd publish && npm publish ${publishTag} --access public --dry-run`, `Testing publish to npm${env === 'beta' ? ' (beta)' : ''} (dry-run)`);
  run(`cd publish && npm pack`, 'Creating package tarball');
}

function publishAllPackages(env, dryRun = false) {
  const workflowType = dryRun ? 'test publish' : 'publish';
  console.log(`\n🚀 Starting full ${workflowType} workflow for ${env} environment...\n`);
  
  // Step 1: Run all tests and checks
  console.log(`\n🧪 Step 1: Running tests and checks...`);
  
  // Run Rust tests (using existing script)
  run('npm run rust:test', 'Running Rust tests');
  
  // Run Rust warnings check (using existing script)  
  run('npm run rust:check-warnings', 'Checking Rust warnings');
  
  // Step 2: Build and publish/test all variants
  console.log(`\n📦 Step 2: ${dryRun ? 'Testing publish' : 'Building and publishing'} all JavaScript packages...`);
  
  const buildConfigs = [
    { target: 'nodejs', variant: 'normal', gc: false },
    { target: 'nodejs', variant: 'normal', gc: true },
    { target: 'browser', variant: 'normal', gc: false },
    { target: 'browser', variant: 'normal', gc: true },
    { target: 'browser', variant: 'inlined', gc: false },
    { target: 'browser', variant: 'inlined', gc: true },
    { target: 'browser', variant: 'asm', gc: false },
    { target: 'browser', variant: 'asm', gc: true }
  ];
  
  const results = [];
  
  for (const config of buildConfigs) {
    const configName = `${config.target}-${config.variant}${config.gc ? '-gc' : ''}`;
    console.log(`\n📦 ${dryRun ? 'Testing' : 'Publishing'} ${configName}...`);
    
    try {
      if (dryRun) {
        testPublishSinglePackage(config, env);
      } else {
        publishSinglePackage(config, env);
      }
      console.log(`✅ Successfully ${dryRun ? 'tested' : 'published'} ${configName}`);
      results.push({ config: configName, status: 'success' });
    } catch (error) {
      console.error(`❌ Failed to ${dryRun ? 'test' : 'publish'} ${configName}: ${error.message}`);
      results.push({ config: configName, status: 'failed', error: error.message });
      // Continue with other packages rather than failing completely
    }
  }
  
  // Step 3: Publish/test Rust crate
  console.log(`\n🦀 Step 3: ${dryRun ? 'Testing Rust crate publish' : 'Publishing Rust crate'}...`);
  try {
    if (dryRun) {
      run('cd rust && cargo publish --dry-run --allow-dirty', 'Testing Rust crate publish (dry-run)');
    } else {
      run('cd rust && cargo publish --allow-dirty', 'Publishing Rust crate to crates.io');
    }
    console.log(`✅ Successfully ${dryRun ? 'tested' : 'published'} Rust crate`);
    results.push({ config: 'rust-crate', status: 'success' });
  } catch (error) {
    console.error(`❌ Failed to ${dryRun ? 'test' : 'publish'} Rust crate: ${error.message}`);
    if (!dryRun) {
      console.log(`   This might be because the version already exists on crates.io`);
    }
    results.push({ config: 'rust-crate', status: 'failed', error: error.message });
  }
  
  // Summary
  console.log(`\n📊 ${dryRun ? 'Test Publish' : 'Publish'} Summary:`);
  const successful = results.filter(r => r.status === 'success');
  const failed = results.filter(r => r.status === 'failed');
  
  console.log(`✅ Successful: ${successful.length}`);
  successful.forEach(r => console.log(`   - ${r.config}`));
  
  if (failed.length > 0) {
    console.log(`❌ Failed: ${failed.length}`);
    failed.forEach(r => console.log(`   - ${r.config}: ${r.error}`));
  }
  
  console.log(`\n🎉 Full ${workflowType} workflow completed!`);
  if (dryRun) {
    console.log(`\n💡 This was a test run. To actually publish, use: build-helper publish-all --env ${env}`);
  }
}

function testPublishAllPackages(env) {
  publishAllPackages(env, true);
}

// Parse arguments and execute commands
program.parse(process.argv); 