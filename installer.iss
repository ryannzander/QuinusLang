; QuinusLang Installer - Inno Setup script
; Build: iscc installer.iss (requires Inno Setup 6)
; Download: https://jrsoftware.org/isdl.php

#define MyAppName "QuinusLang"
#define MyAppVersion "0.1.1"
#define MyAppPublisher "QuinusLang"
#define MyAppExeName "quinus.exe"

[Setup]
AppId={{A1B2C3D4-E5F6-7890-ABCD-EF1234567890}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
AppPublisher={#MyAppPublisher}
DefaultDirName={autopf}\{#MyAppName}
DefaultGroupName={#MyAppName}
AllowNoIcons=yes
OutputDir=installer_output
OutputBaseFilename=QuinusLang-Setup-{#MyAppVersion}
SetupIconFile=
Compression=lzma2
SolidCompression=yes
WizardStyle=modern
PrivilegesRequired=lowest
[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked
Name: "addpath"; Description: "Add QuinusLang to PATH (run quinus from any folder)"; GroupDescription: "Options:"; Flags: checkedonce

[Files]
Source: "quinus.exe"; DestDir: "{app}"; Flags: ignoreversion
#ifexist "runtime.obj"
Source: "runtime.obj"; DestDir: "{app}"; Flags: ignoreversion
#endif
#ifexist "lld-link.exe"
Source: "lld-link.exe"; DestDir: "{app}"; Flags: ignoreversion
#endif
; Include stdlib and compiler for development (optional - skip if missing)
Source: "stdlib\*"; DestDir: "{app}\stdlib"; Flags: ignoreversion recursesubdirs createallsubdirs
Source: "compiler\*"; DestDir: "{app}\compiler"; Flags: ignoreversion recursesubdirs createallsubdirs

[Icons]
Name: "{group}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"
Name: "{group}\{cm:UninstallProgram,{#MyAppName}}"; Filename: "{uninstallexe}"
Name: "{autodesktop}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; Tasks: desktopicon

[Code]
const
  PathKey = 'Environment';

procedure EnvAddPath(Path: string);
var
  Paths: string;
  RootKey: Integer;
begin
  RootKey := HKEY_CURRENT_USER;
  if not RegQueryStringValue(RootKey, PathKey, 'Path', Paths) then Paths := '';
  if Pos(';' + Path + ';', ';' + Paths + ';') > 0 then Exit;
  Paths := Paths + ';' + Path;
  RegWriteStringValue(RootKey, PathKey, 'Path', Paths);
end;

procedure EnvRemovePath(Path: string);
var
  Paths, NewPaths: string;
  P: Integer;
  RootKey: Integer;
begin
  RootKey := HKEY_CURRENT_USER;
  if not RegQueryStringValue(RootKey, PathKey, 'Path', Paths) then Exit;
  NewPaths := Paths;
  P := Pos(';' + Path + ';', ';' + NewPaths + ';');
  if P = 0 then
    P := Pos(';' + Path, ';' + NewPaths + ';');
  if P > 0 then
  begin
    Delete(NewPaths, P, Length(Path) + 1);
    RegWriteStringValue(RootKey, PathKey, 'Path', NewPaths);
  end;
end;

procedure CurStepChanged(CurStep: TSetupStep);
begin
  if (CurStep = ssPostInstall) and WizardIsTaskSelected('addpath') then
    EnvAddPath(ExpandConstant('{app}'));
end;

procedure CurUninstallStepChanged(CurUninstallStep: TUninstallStep);
begin
  if CurUninstallStep = usPostUninstall then
    EnvRemovePath(ExpandConstant('{app}'));
end;

[Run]
Filename: "{app}\{#MyAppExeName}"; Parameters: "--help"; Description: "Show quinus help"; Flags: nowait postinstall skipifsilent

[Messages]
FinishedLabel=Setup has finished installing [name].%n%nIf you added to PATH, close and reopen your terminal for "quinus" to work from any folder.
