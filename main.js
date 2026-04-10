const { app, BrowserWindow, dialog, Menu } = require('electron');
const path = require('path');
const { autoUpdater } = require('electron-updater');

autoUpdater.autoDownload = true;
autoUpdater.on('update-available', () => {
  dialog.showMessageBox({
    type: 'info',
    title: '업데이트 확인',
    message: '새 버전이 있습니다. 백그라운드에서 다운로드합니다.'
  });
});
autoUpdater.on('update-downloaded', () => {
  dialog.showMessageBox({
    type: 'info',
    title: '업데이트 준비 완료',
    message: '업데이트가 준비됐습니다. 재시작 후 적용됩니다.',
    buttons: ['지금 재시작', '나중에']
  }).then(result => {
    if (result.response === 0) autoUpdater.quitAndInstall();
  });
});
autoUpdater.on('error', () => { /* 무시 */ });

let win;
let pinned = false;
let darkMode = false;

function buildMenu() {
  const template = [
    {
      label: app.name,
      submenu: [
        {
          label: '항상 위에 표시',
          type: 'checkbox',
          checked: pinned,
          click: (item) => { pinned = item.checked; if (win) win.setAlwaysOnTop(pinned); }
        },
        { type: 'separator' },
        {
          label: darkMode ? '라이트 모드' : '다크 모드',
          click: () => {
            darkMode = !darkMode;
            if (win) win.webContents.executeJavaScript(
              `document.documentElement.setAttribute('data-theme','${darkMode ? 'dark' : 'light'}')`
            );
            buildMenu();
          }
        },
        { type: 'separator' },
        {
          label: '새로고침',
          accelerator: 'CmdOrCtrl+R',
          click: () => { if (win) win.reload(); }
        },
        { type: 'separator' },
        {
          label: '개발자 도구',
          accelerator: 'CmdOrCtrl+Option+I',
          click: () => { if (win) win.webContents.toggleDevTools(); }
        },
        { type: 'separator' },
        {
          label: '업데이트 확인',
          click: () => autoUpdater.checkForUpdates().catch(() => {})
        },
        {
          label: `v${app.getVersion()}`,
          enabled: false
        }
      ]
    }
  ];
  Menu.setApplicationMenu(Menu.buildFromTemplate(template));
}

function createWindow() {
  win = new BrowserWindow({
    width: 1280,
    height: 800,
    minWidth: 900,
    minHeight: 600,
    titleBarStyle: 'default',
    title: '병원 식대 관리',
    webPreferences: {
      contextIsolation: true,
    },
  });

  win.loadFile('index.html');
  buildMenu();

  win.webContents.on('did-finish-load', () => {
    buildMenu();
  });
}

app.whenReady().then(() => {
  createWindow();

  app.on('activate', () => {
    if (BrowserWindow.getAllWindows().length === 0) createWindow();
  });

  if (app.isPackaged) {
    autoUpdater.checkForUpdatesAndNotify().catch(() => {});
  }
});

app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') app.quit();
});
