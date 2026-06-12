#define MyAppName "Peekdown"
#define MyAppVersion "1.2.1"
#define MyAppPublisher "Peekdown"
#define MyAppURL "https://github.com/silvaivanilto/Peekdown"
#define MyAppExeName "peekdown.exe"

[Setup]
AppId={{C8B4E2D9-0F5A-4B3C-9D8E-1F2A3B4C5D6E}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
AppPublisher={#MyAppPublisher}
AppPublisherURL={#MyAppURL}
DefaultDirName={localappdata}\Programs\{#MyAppName}
DefaultGroupName={#MyAppName}
DisableProgramGroupPage=yes
OutputDir=.
OutputBaseFilename=Peekdown-Setup-{#MyAppVersion}
SetupIconFile=assets\icon.ico
UninstallDisplayIcon={app}\{#MyAppExeName}
Compression=lzma2
SolidCompression=yes
PrivilegesRequired=lowest
DisableWelcomePage=no
WizardStyle=modern
ChangesAssociations=yes

[Languages]
Name: "en"; MessagesFile: "compiler:Default.isl"
Name: "pt"; MessagesFile: "compiler:Languages\BrazilianPortuguese.isl"

[CustomMessages]
en.AssociateGroupDesc=File association:
en.AssociateTaskDesc=Associate .md files with {#MyAppName}
en.DesktopTaskDesc=Create a desktop shortcut

pt.AssociateGroupDesc=Associação de arquivos:
pt.AssociateTaskDesc=Associar arquivos .md ao {#MyAppName}
pt.DesktopTaskDesc=Criar atalho na área de trabalho

[Tasks]
Name: "associate"; Description: {cm:AssociateTaskDesc}; GroupDescription: {cm:AssociateGroupDesc}; Flags: checkedonce
Name: "desktopicon"; Description: {cm:DesktopTaskDesc}; GroupDescription: {cm:AssociateGroupDesc}; Flags: checkedonce

[Files]
Source: "target\release\{#MyAppExeName}"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"
Name: "{group}\{cm:UninstallProgram,{#MyAppName}}"; Filename: "{uninstallexe}"
Name: "{commondesktop}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; Tasks: desktopicon

[Registry]
; Register ProgID
Root: HKCU; Subkey: "Software\Classes\Peekdown.Document"; ValueType: string; ValueName: ""; ValueData: "{#MyAppName} Document"; Flags: uninsdeletekey; Tasks: associate
Root: HKCU; Subkey: "Software\Classes\Peekdown.Document\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\{#MyAppExeName},0"; Flags: uninsdeletekeyifempty; Tasks: associate
Root: HKCU; Subkey: "Software\Classes\Peekdown.Document\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#MyAppExeName}"" ""%1"""; Flags: uninsdeletekeyifempty; Tasks: associate
; Associate .md extension
Root: HKCU; Subkey: "Software\Classes\.md"; ValueType: string; ValueName: ""; ValueData: "Peekdown.Document"; Flags: uninsdeletevalue; Tasks: associate

[Run]
Filename: "{app}\{#MyAppExeName}"; Description: {cm:LaunchProgram,{#MyAppName}}; Flags: postinstall nowait skipifsilent unchecked
