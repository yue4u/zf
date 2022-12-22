; Script generated by the Inno Setup Script Wizard.
; SEE THE DOCUMENTATION FOR DETAILS ON CREATING INNO SETUP SCRIPT FILES!
; vim:ts=2:sw=2:et:

#define MyAppName "WezTerm"
;#define MyAppVersion "1.5"
#define MyAppPublisher "Wez Furlong"
#define MyAppURL "http://wezfurlong.org/wezterm"
#define MyAppExeName "wezterm-gui.exe"

[Setup]
AppId={{BCF6F0DA-5B9A-408D-8562-F680AE6E1EAF}
ArchitecturesAllowed=x64 arm64
ArchitecturesInstallIn64BitMode=x64 arm64
AppName={#MyAppName}
AppVersion={#MyAppVersion}
;AppVerName={#MyAppName} {#MyAppVersion}
AppPublisher={#MyAppPublisher}
AppPublisherURL={#MyAppURL}
AppSupportURL={#MyAppURL}
AppUpdatesURL={#MyAppURL}
DefaultDirName={autopf}\{#MyAppName}
DisableProgramGroupPage=yes
;LicenseFile=..\LICENSE.md
; Remove the following line to run in administrative install mode (install for all users.)
;PrivilegesRequired=lowest
;PrivilegesRequiredOverridesAllowed=dialog
OutputDir=..
OutputBaseFilename=WezTerm-Setup
SetupIconFile=..\assets\windows\terminal.ico
UninstallDisplayIcon={app}\{#MyAppExeName}
Compression=lzma
SolidCompression=yes
WizardStyle=modern
; Build 1809 is required for pty support
MinVersion=10.0.17763
ChangesEnvironment=true

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked

[Files]
Source: "..\target\release\wezterm.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\target\release\wezterm-gui.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\target\release\wezterm-mux-server.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\target\release\mesa\opengl32.dll"; DestDir: "{app}\mesa"; Flags: ignoreversion
Source: "..\target\release\libEGL.dll"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\target\release\libGLESv2.dll"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\target\release\conpty.dll"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\target\release\OpenConsole.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\target\release\strip-ansi-escapes.exe"; DestDir: "{app}"; Flags: ignoreversion
; NOTE: Don't use "Flags: ignoreversion" on any shared system files

[Icons]
Name: "{autoprograms}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; AppUserModelID: "org.wezfurlong.wezterm"
Name: "{autodesktop}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; Tasks: desktopicon; AppUserModelID: "org.wezfurlong.wezterm"

[Run]
Filename: "{app}\{#MyAppExeName}"; Description: "{cm:LaunchProgram,{#StringChange(MyAppName, '&', '&&')}}"; Flags: nowait postinstall skipifsilent

[Registry]
Root: HKA; Subkey: "Software\Classes\Drive\shell\Open WezTerm here"; Flags: uninsdeletekey
Root: HKA; Subkey: "Software\Classes\Drive\shell\Open WezTerm here"; ValueName: "icon"; ValueType: string; ValueData: "{app}\{#MyAppExeName}"; Flags: uninsdeletekey;
Root: HKA; Subkey: "Software\Classes\Drive\shell\Open WezTerm here\command"; ValueType: string; ValueData: """{app}\{#MyAppExeName}"" start --no-auto-connect --cwd ""%V\"""; Flags: uninsdeletekey;
Root: HKA; Subkey: "Software\Classes\Directory\Background\shell\Open WezTerm here"; Flags: uninsdeletekey
Root: HKA; Subkey: "Software\Classes\Directory\Background\shell\Open WezTerm here"; ValueName: "icon"; ValueType: string; ValueData: "{app}\{#MyAppExeName}"; Flags: uninsdeletekey;
Root: HKA; Subkey: "Software\Classes\Directory\Background\shell\Open WezTerm here\command"; ValueType: string; ValueData: """{app}\{#MyAppExeName}"" start --no-auto-connect --cwd ""%V"; Flags: uninsdeletekey;
Root: HKA; Subkey: "Software\Classes\Directory\shell\Open WezTerm here"; Flags: uninsdeletekey
Root: HKA; Subkey: "Software\Classes\Directory\shell\Open WezTerm here"; ValueName: "icon"; ValueType: string; ValueData: "{app}\{#MyAppExeName}"; Flags: uninsdeletekey;
Root: HKA; Subkey: "Software\Classes\Directory\shell\Open WezTerm here\command"; ValueType: string; ValueData: """{app}\{#MyAppExeName}"" start --no-auto-connect --cwd ""%V\\"""; Flags: uninsdeletekey;

[Code]
{ https://stackoverflow.com/a/46609047/149111 }
const EnvironmentKey = 'SYSTEM\CurrentControlSet\Control\Session Manager\Environment';

const IMAGE_FILE_MACHINE_AMD64 = $8664;

function GetMachineTypeAttributes(
    Machine: Word; var MachineTypeAttributes: Integer): HRESULT;
  external 'GetMachineTypeAttributes@Kernel32.dll stdcall delayload';

function IsSupportedArch(): Boolean;
var
  Version: TWindowsVersion;
  MachineTypeAttributes: Integer;
  Arch: TSetupProcessorArchitecture;
begin
  GetWindowsVersionEx(Version);
  // Use capability detection if available. GetMachineTypeAttributes is
  // available starting Build 22000
  if Version.Build >= 22000 then
  begin
    OleCheck(
      GetMachineTypeAttributes(IMAGE_FILE_MACHINE_AMD64, MachineTypeAttributes)
    );
    Result := MachineTypeAttributes <> 0;
  end
  else
  begin
    // If capability detection is not available, then we rely on build number.
    // x64 emulation on arm64 is available starting build 21277 so both x64 and
    // arm64 will work
    if Version.Build >= 21277 then
    begin
      Result := True;
    end
    else
    begin
      // If we're here, it means we're on a build between 17763 and 21277
      // because Inno will check MinVersion 10.0.17763 and arch will be x64 or
      // arm64. Only x64 is supported in this build range
      Arch := ProcessorArchitecture;
      Result := Arch = paX64;
    end
  end;
end;

<event('InitializeSetup')>
function InitializeSetupCheckArchitecture(): Boolean;
begin
  Result := IsSupportedArch();
end;

procedure EnvAddPath(instlPath: string);
var
  Paths: string;
begin
  { Retrieve current path (use empty string if entry not exists) }
  if not RegQueryStringValue(HKEY_LOCAL_MACHINE, EnvironmentKey, 'Path', Paths) then
    Paths := '';

  if Paths = '' then
    Paths := instlPath + ';'
  else
  begin
    { Skip if string already found in path }
    if Pos(';' + Uppercase(instlPath) + ';',  ';' + Uppercase(Paths) + ';') > 0 then exit;
    if Pos(';' + Uppercase(instlPath) + '\;', ';' + Uppercase(Paths) + ';') > 0 then exit;

    { Append App Install Path to the end of the path variable }
    if Paths[length(Paths)] <> ';' then
      Paths := Paths + ';';

    Paths := Paths + instlPath + ';';
  end;

  { Overwrite (or create if missing) path environment variable }
  if RegWriteStringValue(HKEY_LOCAL_MACHINE, EnvironmentKey, 'Path', Paths)
  then Log(Format('The [%s] added to PATH: [%s]', [instlPath, Paths]))
  else Log(Format('Error while adding the [%s] to PATH: [%s]', [instlPath, Paths]));
end;

procedure EnvRemovePath(instlPath: string);
var
  Paths: string;
  P, Offset, DelimLen: Integer;
begin
  { Skip if registry entry not exists }
  if not RegQueryStringValue(HKEY_LOCAL_MACHINE, EnvironmentKey, 'Path', Paths) then
    exit;

  { Skip if string not found in path }
  DelimLen := 1;     { Length(';') }
  P := Pos(';' + Uppercase(instlPath) + ';', ';' + Uppercase(Paths) + ';');
  if P = 0 then
  begin
    { perhaps instlPath lives in Paths, but terminated by '\;' }
    DelimLen := 2; { Length('\;') }
    P := Pos(';' + Uppercase(instlPath) + '\;', ';' + Uppercase(Paths) + ';');
    if P = 0 then exit;
  end;

  { Decide where to start string subset in Delete() operation. }
  if P = 1 then
    Offset := 0
  else
    Offset := 1;
  { Update path variable }
  Delete(Paths, P - Offset, Length(instlPath) + DelimLen);

  { Overwrite path environment variable }
  if RegWriteStringValue(HKEY_LOCAL_MACHINE, EnvironmentKey, 'Path', Paths)
  then Log(Format('The [%s] removed from PATH: [%s]', [instlPath, Paths]))
  else Log(Format('Error while removing the [%s] from PATH: [%s]', [instlPath, Paths]));
end;

procedure CurStepChanged(CurStep: TSetupStep);
begin
  if CurStep = ssPostInstall then
    EnvAddPath(ExpandConstant('{app}'));
end;

procedure CurUninstallStepChanged(CurUninstallStep: TUninstallStep);
begin
  if CurUninstallStep = usPostUninstall then
    EnvRemovePath(ExpandConstant('{app}'));
end;
