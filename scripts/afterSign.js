const { execSync } = require('child_process');
const path = require('path');
const fs = require('fs');

module.exports = async function afterSign(context) {
  const { electronPlatformName, appOutDir, packager } = context;

  if (electronPlatformName !== 'darwin') return;

  const appName = packager.appInfo.productFilename;
  const appPath = path.join(appOutDir, `${appName}.app`);
  const entitlements = path.join(__dirname, '..', 'entitlements.plist');

  const sign = (target) => {
    execSync(
      `codesign --force --sign - --entitlements "${entitlements}" "${target}"`,
      { stdio: 'inherit' }
    );
  };

  const frameworkBase = path.join(
    appPath,
    'Contents/Frameworks/Electron Framework.framework/Versions/A'
  );

  // Sign dylibs inside Electron Framework
  const libDir = path.join(frameworkBase, 'Libraries');
  if (fs.existsSync(libDir)) {
    for (const f of fs.readdirSync(libDir)) {
      if (f.endsWith('.dylib')) sign(path.join(libDir, f));
    }
  }

  // Sign Electron Framework bundle
  sign(path.join(appPath, 'Contents/Frameworks/Electron Framework.framework'));

  // Sign all helper apps
  const fwDir = path.join(appPath, 'Contents/Frameworks');
  for (const entry of fs.readdirSync(fwDir)) {
    if (entry.endsWith('.app')) {
      sign(path.join(fwDir, entry));
    }
  }

  // Sign main app last
  sign(appPath);
};
